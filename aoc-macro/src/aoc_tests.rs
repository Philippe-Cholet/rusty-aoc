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
    pub year_tests: Vec<(u8, Vec<[Option<String>; 2]>)>,
}

/// `_` OR `"some text"` standing for `Option<String>`.
enum Answer {
    Zero(Token![_]),
    OneStr(LitStr),
    OneInt(LitInt),
}

/// Answer OR `(_, "some text")` standing for `[Option<String>; 2]`.
enum Answers {
    MaxOne(Answer),
    Multiple(Punctuated<Answer, Token![,]>),
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

impl Parse for Answers {
    fn parse(input: ParseStream) -> Result<Self> {
        let res = if input.fork().parse::<Answer>().is_ok() {
            Self::MaxOne(input.parse::<Answer>()?)
        } else {
            let content;
            parenthesized!(content in input);
            let answers = content.parse_terminated(Answer::parse)?;
            Self::Multiple(answers)
        };
        input.parse::<Nothing>()?;
        Ok(res)
    }
}

impl TryFrom<Answers> for [Option<String>; 2] {
    type Error = Error;

    fn try_from(value: Answers) -> Result<Self> {
        Ok(match value {
            Answers::MaxOne(a) => [a.into(), None],
            Answers::Multiple(value) => {
                let mut answers = value.into_iter();
                let Some(a1) = answers.next() else {
                    return Ok([None, None]);
                };
                let Some(a2) = answers.next() else {
                    return Ok([a1.into(), None]);
                };
                if let Some(a3) = answers.next() {
                    let span = match a3 {
                        Answer::Zero(u) => u.span,
                        Answer::OneStr(s) => s.span(),
                        Answer::OneInt(i) => i.span(),
                    };
                    return Err(Error::new(span, "More than two answers?!"));
                }
                [a1.into(), a2.into()]
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
            let answers = content.parse_terminated(Answers::parse)?;
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
