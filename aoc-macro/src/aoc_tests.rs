use syn::{
    parenthesized,
    parse::{Nothing, Parse, ParseStream},
    punctuated::Punctuated,
    Error, Ident, LitInt, LitStr, Result, Token,
};

#[derive(Debug)]
/// The name, and for each year, two optional answers for each day.
pub struct AocTests {
    pub name: String,
    pub year_tests: Vec<(u8, Vec<[UnitTest; 2]>)>,
}

#[derive(Debug, Default)]
pub struct Test<T> {
    pub data: T,
    pub ignore: bool,
}

type UnitTest = Test<Option<String>>;

impl UnitTest {
    pub const fn skip(&self, ignore: bool) -> bool {
        self.data.is_none() || self.ignore != ignore
    }
}

/// `_` OR `"some text"` standing for `Option<String>`.
enum Answer {
    Zero(Token![_]),
    OneStr(LitStr),
    OneInt(LitInt),
}

/// Answer OR `(_, "some text")` standing for `[UnitTest; 2]`.
enum Answers {
    MaxOne(Test<Answer>),
    Multiple(Test<Punctuated<Test<Answer>, Token![,]>>),
}

/// `name, 21 => (/*tests21*/), 22 => (/*tests22*/), ...` standing for `AocTests`.
struct SynAocTests {
    name: Ident,
    year_tests: Vec<(LitInt, Punctuated<Answers, Token![,]>)>,
}

impl Parse for Answer {
    fn parse(input: ParseStream) -> Result<Self> {
        let res = if input.peek(Token![_]) {
            Self::Zero(input.parse()?)
        } else if input.peek(LitStr) {
            Self::OneStr(input.parse()?)
        } else {
            Self::OneInt(input.parse()?)
        };
        input.parse::<Nothing>()?;
        Ok(res)
    }
}

impl From<Answer> for Option<String> {
    fn from(value: Answer) -> Self {
        match value {
            Answer::Zero(_) => None,
            Answer::OneStr(s) => Some(s.value()),
            Answer::OneInt(i) => Some(i.to_string()),
        }
    }
}

impl Test<Answer> {
    fn into_or(self, ignore: bool) -> UnitTest {
        Test {
            data: self.data.into(),
            ignore: self.ignore || ignore,
        }
    }
}

impl<T> Test<T> {
    fn parse<F>(input: ParseStream, func: F) -> Result<Self>
    where
        F: FnOnce(ParseStream) -> Result<T>,
    {
        let ignore = match input.fork().parse::<Ident>() {
            Ok(ident) if ident.to_string().eq("ignore") => {
                input.parse::<Ident>()?;
                true
            }
            _ => false,
        };
        func(input).map(|data| Self { data, ignore })
    }
}

impl Parse for Answers {
    fn parse(input: ParseStream) -> Result<Self> {
        let res = if input.fork().parse::<Answer>().is_ok() {
            Self::MaxOne(Test::parse(input, Answer::parse)?)
        } else {
            Self::Multiple(Test::parse(input, |input0| {
                let content;
                parenthesized!(content in input0);
                content.parse_terminated(|input1| Test::parse(input1, Answer::parse), Token![,])
            })?)
        };
        input.parse::<Nothing>()?;
        Ok(res)
    }
}

impl TryFrom<Answers> for [UnitTest; 2] {
    type Error = Error;

    fn try_from(value: Answers) -> Result<Self> {
        Ok(match value {
            Answers::MaxOne(a) => [a.into_or(false), Test::default()],
            Answers::Multiple(Test { data, ignore }) => {
                let mut answers = data.into_iter();
                let Some(a1) = answers.next() else {
                    return Ok([Test::default(), Test::default()]);
                };
                let Some(a2) = answers.next() else {
                    return Ok([a1.into_or(ignore), Test::default()]);
                };
                if let Some(a3) = answers.next() {
                    let span = match a3.data {
                        Answer::Zero(u) => u.span,
                        Answer::OneStr(s) => s.span(),
                        Answer::OneInt(i) => i.span(),
                    };
                    return Err(Error::new(span, "More than two answers?!"));
                }
                [a1.into_or(ignore), a2.into_or(ignore)]
            }
        })
    }
}

impl Parse for SynAocTests {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        let mut year_tests = vec![];
        while !input.is_empty() {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
            let year = input.parse()?;
            input.parse::<Token![=>]>()?;
            let content;
            parenthesized!(content in input);
            let answers = content.parse_terminated(Answers::parse, Token![,])?;
            year_tests.push((year, answers));
        }
        // input.parse::<Nothing>()?; // It is empty so it is not needed.
        Ok(Self { name, year_tests })
    }
}

impl TryFrom<SynAocTests> for AocTests {
    type Error = Error;

    fn try_from(value: SynAocTests) -> Result<Self> {
        let name = value.name.to_string();
        let mut year_tests = vec![];
        for (year, tests) in value.year_tests {
            let year = year.base10_parse()?;
            let mut ts = vec![];
            for answers in tests {
                ts.push(answers.try_into()?);
            }
            year_tests.push((year, ts));
        }
        Ok(Self { name, year_tests })
    }
}

impl Parse for AocTests {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<SynAocTests>()?.try_into()
    }
}
