#[derive(Debug, PartialEq, Eq)]
pub enum MatcherToken<'a> {
    RawText(&'a str),
    OneOfText(Vec<&'a str>),
    WildCard,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Matcher<'a> {
    text: &'a str,
    tokens: Vec<MatcherToken<'a>>,
    most_tokens_matched: usize,
}

impl<'a> Matcher<'a> {
    pub fn new(text: &'a str) -> Option<Matcher<'a>> {
        let mut tokens = vec![];
        let mut text_left = text;

        loop {
            if text_left.is_empty() {
                break;
            } else if text_left.starts_with('.') {
                tokens.push(MatcherToken::WildCard);
                text_left = &text_left[1..];
            } else if text_left.starts_with('(') {
                let first_close = text_left.find(')')?;
                let (options, leftover) = text_left.split_at(first_close);
                tokens.push(MatcherToken::OneOfText(options[1..].split('|').collect()));
                text_left = &leftover[1..];
            } else {
                let first_wc = text_left.find('.').unwrap_or(text_left.len());
                let first_one_of = text_left.find('(').unwrap_or(text_left.len());
                let first_token = first_wc.min(first_one_of);
                tokens.push(MatcherToken::RawText(&text_left[..first_token]));
                text_left = &text_left[first_token..];
            }
        }

        Some(Matcher {
            text,
            tokens,
            most_tokens_matched: 0,
        })
    }

    pub fn text(&self) -> &'a str {
        self.text
    }

    pub fn most_tokens_matched(&self) -> usize {
        self.most_tokens_matched
    }

    pub fn match_string<'m, 'b>(
        &'m mut self,
        string: &'b str,
    ) -> Vec<(&'m MatcherToken<'a>, &'b str)> {
        let mut string_left = string;
        let mut answer = vec![];

        'outer_loop: for token in self.tokens.iter() {
            if string_left.is_empty() {
                break;
            }

            match token {
                MatcherToken::WildCard => {
                    let byte_offset = string_left.chars().next().unwrap().len_utf8();
                    answer.push((token, &string_left[..byte_offset]));
                    string_left = &string_left[byte_offset..];
                }
                MatcherToken::OneOfText(options) => {
                    for start in options {
                        if string_left.starts_with(start) {
                            answer.push((token, &string_left[..start.len()]));
                            string_left = &string_left[start.len()..];
                            continue 'outer_loop;
                        }
                    }
                    break;
                }
                MatcherToken::RawText(text) => {
                    if string_left.starts_with(text) {
                        answer.push((token, &string_left[..text.len()]));
                        string_left = &string_left[text.len()..];
                    } else {
                        break;
                    }
                }
            }
        }

        self.most_tokens_matched = self.most_tokens_matched.max(answer.len());
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::{Matcher, MatcherToken};

    #[test]
    fn matches_tokens_and_tracks_best_result() {
        let match_string = "abc(d|e|f).".to_string();
        let mut matcher = Matcher::new(&match_string).unwrap();

        assert_eq!(matcher.most_tokens_matched(), 0);

        {
            let candidate = "abcge".to_string();
            let result = matcher.match_string(&candidate);
            assert_eq!(result, vec![(&MatcherToken::RawText("abc"), "abc")]);
            assert_eq!(matcher.most_tokens_matched(), 1);
        }

        {
            let candidate = "abcde".to_string();
            let result = matcher.match_string(&candidate);
            assert_eq!(
                result,
                vec![
                    (&MatcherToken::RawText("abc"), "abc"),
                    (&MatcherToken::OneOfText(vec!["d", "e", "f"]), "d"),
                    (&MatcherToken::WildCard, "e"),
                ]
            );
            assert_eq!(matcher.most_tokens_matched(), 3);
        }
    }

    #[test]
    fn wildcard_consumes_one_unicode_character() {
        let pattern = "a.".to_string();
        let candidate = "a💪".to_string();
        let mut matcher = Matcher::new(&pattern).unwrap();

        let result = matcher.match_string(&candidate);

        assert_eq!(result[1].1, "💪");
        assert_eq!(matcher.most_tokens_matched(), 2);
    }

    #[test]
    fn rejects_unclosed_options() {
        let match_string = "abc(d|e|f.".to_string();

        assert_eq!(Matcher::new(&match_string), None);
    }
}
