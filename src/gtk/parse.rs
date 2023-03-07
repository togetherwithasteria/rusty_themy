use std::convert::Infallible;
use std::io::Read;
use std::{fs::File, path::Path};

use cssparser::*;

use lightningcss::error::PrinterErrorKind;
use lightningcss::printer::Printer;
use lightningcss::rules::CssRule;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use lightningcss::traits::AtRuleParser;
use lightningcss::traits::ToCss;
use lightningcss::values::ident::Ident;
use lightningcss::visit_types;
use lightningcss::visitor::{Visit, VisitTypes, Visitor};

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ParserError(String),
}

pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Vec<DefineColor>, Error> {
    let mut local_config = File::open(path).map_err(|e| Error::IoError(e))?;
    let mut string = String::new();

    local_config
        .read_to_string(&mut string)
        .map_err(|e| Error::IoError(e))?;

    from_str(&string)
}
pub fn from_str(css: &str) -> Result<Vec<DefineColor>, Error> {
    let mut defined_colors = vec![];
    match StyleSheet::parse_with(
        &css,
        ParserOptions {
            error_recovery: true,
            ..Default::default()
        },
        &mut ColorParser,
    ) {
        Ok(mut stylesheet) => {
            stylesheet
                .visit(&mut DefineColorCollector {
                    colors: &mut defined_colors,
                })
                .unwrap();
        }
        Err(e) => return Err(Error::ParserError(e.to_string())),
    };
    Ok(defined_colors)
}

struct ColorParser;
#[derive(Debug, Clone)]
enum Prelude<'i> {
    DefineColor(Ident<'i>, Color),
}
#[derive(Debug, Clone)]
enum AtRule {
    DefineColor(DefineColor),
}
#[derive(Debug, Clone, PartialEq)]
pub struct DefineColor {
    pub ident: String,
    pub color: Color,
    pub loc: SourceLocation,
}
#[derive(Debug, Clone)]
struct DefineColorDirective {
    pub ident: String,
}

impl<'i> AtRuleParser<'i> for ColorParser {
    type Prelude = Prelude<'i>;

    type AtRule = AtRule;

    type Error = Infallible;

    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input_parser: &mut Parser<'i, 't>,
        _: &ParserOptions<'_, 'i>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        match_ignore_ascii_case! { &name,
                    "define-color" => {

        let mut ident = None;
        let mut color = None;
        for count in 1..3 {
            println!("Hiii2");
          if let Ok(_) = input_parser.try_parse(|input| {
             if count == 1 {
                println!("Hiii3");
                ident = Some(input.expect_ident_cloned());
            } else if count == 2 {
                color = Some(cssparser::Color::parse(input));
                println!("Hiii4");
            };

            Result::<(), ()>::Ok(())
          }) {

          } else {
            break
          }
        }




        match color.unwrap() {
            Ok(color) =>    Ok(Prelude::DefineColor(Ident(ident.unwrap().unwrap().into()), color)),
            Err(err) => Err(err.into())
                     } },

                    _ => Err(input_parser.new_error(BasicParseErrorKind::AtRuleInvalid(name)))
                }
    }

    fn rule_without_block(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
        _: &ParserOptions<'_, 'i>,
    ) -> Result<Self::AtRule, ()> {
        println!("Hiiii");
        match prelude {
            Prelude::DefineColor(ident, color) => Ok(AtRule::DefineColor(DefineColor {
                ident: ident.to_string(),
                color,
                loc: start.source_location(),
            })),
        }
    }
}

struct DefineColorCollector<'i> {
    colors: &'i mut Vec<DefineColor>,
}

impl<'i, V: Visitor<'i, AtRule>> Visit<'i, AtRule, V> for AtRule {
    const CHILD_TYPES: VisitTypes = VisitTypes::empty();

    fn visit_children(&mut self, _: &mut V) -> Result<(), V::Error> {
        Ok(())
    }
}

impl<'a, 'i> Visitor<'i, AtRule> for DefineColorCollector<'i> {
    type Error = Infallible;

    const TYPES: VisitTypes = visit_types!(RULES);

    fn visit_rule(&mut self, rule: &mut CssRule<'i, AtRule>) -> Result<(), Self::Error> {
        println!("Hiii");
        if let CssRule::Custom(AtRule::DefineColor(color)) = rule {
            self.colors.push(color.clone());
        }

        Ok(())
    }

    fn visit_color(
        &mut self,
        color: &mut lightningcss::values::color::CssColor,
    ) -> Result<(), Self::Error> {
        *color = color.to_lab();
        Ok(())
    }
}

impl ToCss for AtRule {
    fn to_css<W: std::fmt::Write>(
        &self,
        _: &mut Printer<'_, '_, '_, W>,
    ) -> Result<(), lightningcss::error::Error<PrinterErrorKind>> {
        unimplemented!()
    }
}
#[cfg(test)]
pub mod test {
    use cssparser::{Color, SourceLocation, RGBA};

    use super::DefineColor;

    use super::from_str;
    #[test]
    pub fn test() {
        let css = from_str(include_str!("gtk.css")).unwrap();

        assert_eq!(
            *css.first().unwrap(),
            DefineColor {
                ident: "accent_color".to_string(),
                color: Color::RGBA(RGBA {
                    red: 233,
                    green: 70,
                    blue: 134,
                    alpha: 255,
                }),
                loc: SourceLocation { line: 2, column: 1 },
            },
        );

        assert_eq!(
            *css.last().unwrap(),
            DefineColor {
                ident: "cute_fg".to_string(),
                color: Color::RGBA(RGBA {
                    red: 191,
                    green: 16,
                    blue: 76,
                    alpha: 255,
                }),
                loc: SourceLocation {
                    line: 80,
                    column: 1,
                },
            },
        )
    }
}
