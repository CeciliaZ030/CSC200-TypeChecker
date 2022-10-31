use std::cmp::PartialEq;
use std::clone::Clone;
use std::collections::HashMap;

#[derive(Clone)]
pub enum State {
	Sstart,
	Sstr,
	Snum,
	Spunc,
}

#[derive(Debug, Clone)]
pub enum Token {
    Tnum(i32), /* number */
	Tid(String), /* id */
    Ttype(Type),

	// keywords
    TnumC,
    TplusC,
    TmultC,
	TtrueC,
	TfalseC,
	TeqC,
	TifC,
	TidC,
	TappC,
	TfdC,
	TrecC,

	// punctuations
	TleftParen,
	TrightParen,
	Tcomma,
    Tquote,
}

/* allow comparsion between states */
impl PartialEq for State {
    	fn eq(&self, other: &State) -> bool {
       		match (self, other) {
        		(&State::Sstr, &State::Sstr) |
            		(&State::Snum, &State::Snum) |
        		(&State::Spunc, &State::Spunc) |
            		(&State::Sstart, &State::Sstart) => true,
        		_ => false,
        }
    }
}

/* allow comparsion between tokens */
impl PartialEq for Token {
	fn eq(&self, other: &Token) -> bool {
		match (self, other) {
			(&Token::TleftParen, &Token::TleftParen) |
			(&Token::TrightParen, &Token::TrightParen) |
			(&Token::Tcomma, &Token::Tcomma) |
            (&Token::Tquote, &Token::Tquote) => true,
			_ => false,
		}
	}
}

/* --------------- Tokenizer Function ---------------
 * @parameter: a string that is in TyExprC language
 * @return: a vector of tokens
 * Description: the function breaks down the input string and
 * 		 		creates a list containing tokens.
 */
pub fn tokenize(input: String) -> Vec<Token> {

	let mut state;
	let mut last_state = State::Sstart;

	let mut buffer = String::new();
	let mut token_list: Vec<Token> = Vec::new();

	let input: String = input.split_whitespace().collect();
	let input_vec: Vec<char> = input.chars().collect();

	for i in 0 .. (input_vec.len() + 1) {
		let c;
		if i < input.len() {
			c = input_vec[i];
		} else {
			c = '#';
		}
		match c {
			'a' ..= 'z' | 'A' ..= 'Z' => {
				state = State::Sstr;
				/*
				Sstr -a-> Sstr
				Snum -a-> Sstr
				Spunc -a-> Sstr, and flush
				S0 -a-> Sstr
				*/
				if last_state == State::Spunc {
					bind_token(&buffer, &mut token_list, &last_state);
					buffer = String::new();
				}
				last_state = state.clone();
			},
			'0' ..= '9' => {
				state = State::Snum;
				/*
				Sstr -1-> Sstr
				Snum -1-> Snum
				Spunc -1-> Snum, and flush
				S0 -1-> Snum
				*/
				if state == State::Sstart {
					state = State::Snum;
				}
				if last_state == State::Sstr{
					state = State::Sstr;
				}
				if last_state == State::Spunc {
					bind_token(&buffer, &mut token_list, &last_state);
					buffer = String::new();

				}
				last_state = state.clone();
				// if last_state == State::Spunc { println!("emm..."); }
			},
			'(' | ')' | '"' | ',' => {
				state = State::Spunc;
				/*
				Sstr -,-> Spunc
				Snum -,-> Spunc
				Spunc -,-> Spunc
				S0 -,-> panic!
				*/
				// if last_state == State::Sstart {
				// 	panic!("Start with {} ?", c);
				// }
				bind_token(&buffer, &mut token_list, &last_state);
				buffer = String::new();
				last_state = state.clone();
			},
			'#' => {
				if buffer.chars().last().unwrap() != ')' && i == input_vec.len() - 1 {
					panic!("{} at the end", c);
				}
				bind_token(&buffer, &mut token_list, &last_state);
			},
			_ => println!("{} what is this char ?", c),
		}
		buffer.push_str(&c.to_string());
	}
	token_list
}

pub fn bind_token(input: &String, token_list: &mut Vec<Token>, state: &State){
	let s = input.as_str();
	match state {
		State::Spunc => {
			// println!("Spunc, insert {}", input);
			match s {
				"(" => token_list.push(Token::TleftParen),
				")" => token_list.push(Token::TrightParen),
				"\"" => token_list.push(Token::Tquote),
				"," => token_list.push(Token::Tcomma),
				_=> panic!("Unexpected punctuation: {}", input),
			}
		}
		State::Snum => {
			// println!("Snum, insert {}", input);
			token_list.push(Token::Tnum(input.parse::<i32>().unwrap()));
		}
		State::Sstr => {
			println!("Sstr, insert {}", input);
			match s {
				"numC" => token_list.push(Token::TnumC),
				"plusC" => token_list.push(Token::TplusC),
				"multC" => token_list.push(Token::TmultC),
				"trueC" => token_list.push(Token::TtrueC),
				"falseC" => token_list.push(Token::TfalseC),
				"eqC" => token_list.push(Token::TeqC),
				"ifC" => token_list.push(Token::TifC),
				"idC" => token_list.push(Token::TidC),
				"appC" => token_list.push(Token::TappC),
				"fdC" => token_list.push(Token::TfdC),
				"recC" => token_list.push(Token::TrecC),
				"numT"=> token_list.push(Token::Ttype(Type::NumT)),
				"boolT" => token_list.push(Token::Ttype(Type::BoolT)),
				// "funT" => token_list.push(Token::Ttype(Type::FunT(Box::new(), Box::new()))),
				_ => token_list.push(Token::Tid(s.to_string())),
			}
		}
		_=> panic!("..."),
	}
}

/* ====================================================================================== */
/* ====================================================================================== */
/* ====================================================================================== */
/* ====================================================================================== */
/* ====================================================================================== */

#[derive(Debug, Clone)]
pub enum Type {
	NumT,
	BoolT,
	FunT(Box<Type>, Box<Type>),
}

impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool {
        let first = self.clone();
        let second = other.clone();
		match (first, second) {
			(Type::NumT, Type::NumT) |
			(Type::BoolT, Type::BoolT) => true,
			(Type::FunT(_a, _b), Type::FunT(_x, _y)) => true,
			_ => false,
		}
	}
}

pub fn is_type(check_type: &Token) -> bool {
    match check_type {
        Token::Ttype(_t) => true,
        _ => false,
    }
}

pub enum AST {
	AnumC(i32),
    AidC(String),
	AplusC(Box<AST>, Box<AST>),
	AmultC(Box<AST>, Box<AST>),
	AtrueC(bool),
	AfalseC(bool),
	AifC(Box<AST>, Box<AST>, Box<AST>),
	AappC(Box<AST>, Box<AST>),
	AfdC(String, Box<Type>, Box<Type>, Box<AST>),
	AeqC(Box<AST>, Box<AST>),
	ArecC(String, String, Box<Type>, Box<Type>, Box<AST>, Box<AST>),
	None /* for Token::Tcomma and Token::Tquote */ ,
    AtypeC(Type),
}

/* --------------- Parser Function ---------------
 * @parameter: a vector of tokens
 * @return: an AST
 * Description: the function matches the token one-by-one, and check
 * if there is any violation of the TyExprC format.
 */
pub fn parse(tk_list: &[Token]) -> AST {
	// println!("Parameter {:?}", tk_list);
	if tk_list.len() == 1 &&
    /* punctuations */
	(tk_list[0] == Token::TleftParen || tk_list[0] == Token::TrightParen || tk_list[0] == Token::Tcomma || tk_list[0] == Token::Tquote) ||
    /* keywords */
    (tk_list[0] == Token::TnumC || tk_list[0] == Token::TplusC || tk_list[0] == Token::TmultC || tk_list[0] == Token::TeqC || tk_list[0] == Token::TifC || tk_list[0] == Token::TidC || tk_list[0] == Token::TappC || tk_list[0] == Token::TrecC) {
		panic!("Invalid Token (length = 1).")
	}

    let check_first = tk_list[0].clone();
	match check_first {
		/* ========== Base Cases ========== */
		Token::Tnum(i) => {
			if tk_list.len() != 1 {
				panic!("Invalid Character after Number.")
			}
			AST::AnumC(i)
		},
		Token::Tid(s) => {
			if tk_list.len() != 1 {
				panic!("Parsing Error Occurred.")
			}
            AST::AidC(s)
		},
        Token::Ttype(t) => AST::AtypeC(t),

		Token::TtrueC => {
			if tk_list.len() != 1 {
				panic!("Invalid Character after \"true\" Token.")
			}
			AST::AtrueC(true)
		},
		Token::TfalseC => {
			if tk_list.len() != 1 {
				panic!("Invalid Character after \"false\" Token.")
			}
			AST::AfalseC(false)
		},

        /* ========== Punctuation Cases ========== */
		// TODO: maybe not needed?
		// Token::TleftParen => {
		// 	if tk_list[1] == Token::TleftParen {
		// 		panic!("Multiple Left Parentheses.")
		// 	} else if tk_list[1] == Token::TrightParen {
		// 		panic!("Empty Value in two Parentheses.")
		// 	}
		// 	if tk_list[tk_list.len() - 1] == Token::TrightParen {
		// 		parse(&tk_list[1 .. tk_list.len() - 1])
		// 	} else {
		// 		panic!("Left Parenthesis Failed.")
		// 	}
		// },
		Token::TleftParen | Token::TrightParen => {
			panic!("Cannot Have Parenthesis as First Element.")
		},
		Token::Tcomma | Token::Tquote => {
			AST::None
		},

		/* ========== Keyword Cases ========== */
		Token::TnumC => {
			if tk_list[1] != Token::TleftParen {
				panic!("Missing Left Parenthesis.")
			} else if tk_list[tk_list.len() - 1] != Token::TrightParen {
				panic!("Missing Right Parenthesis.")
			} else if tk_list.len() != 4 {
                panic!("Incorrect format of NumC (Incorrect Length).")
            } else {
                let temp = tk_list[2].clone();
                let num: &[Token] = &vec![temp];
				parse(num)
			}
		} /* [END] Token::TnumC */ ,

    	Token::TidC => {
			if tk_list[1] != Token::TleftParen {
				panic!("Missing Left Parenthesis.")
			} else if tk_list[tk_list.len() - 1] != Token::TrightParen {
				panic!("Missing Right Parenthesis.")
			} else {
                if tk_list.len() == 4 {
                    let temp = tk_list[2].clone();
                    let s: &[Token] = &vec![temp];
                    parse(s)
                } else if tk_list.len() == 6 {
                    if tk_list[2] == Token::Tquote && tk_list[4] == Token::Tquote {
                        let temp = tk_list[3].clone();
                        let s: &[Token] = &vec![temp];
                        parse(s)
                    } else {
                        panic!("Incorrect Format of idC (length).")
                    }
                } else {
                    panic!("Incorrect Format of idC (length).")
                }
        	}
		} /* [END] Token::TidC */ ,

		Token::TplusC => {
            if tk_list.len() < 6 {
                panic!("Incorrect format of plusC")
            }
			if tk_list[1] != Token::TleftParen {
				panic!("Missing Left Parenthesis.")
			}
			if tk_list[tk_list.len() - 1] != Token::TrightParen {
				panic!("Missing Right Parentesis.")
			}

            /* plusC(x, y) */
			if tk_list.len() == 6 && tk_list[3] == Token::Tcomma {
                let temp = tk_list[2].clone();
				let first: &[Token] = &vec![temp];
                let temp = tk_list[4].clone();
				let second: &[Token] = &vec![temp];
				AST::AplusC(Box::new(parse(first)), Box::new(parse(second)))
			}

            /* plusC(numC(x), numC(y)) */
            else if tk_list.len() == 12 && tk_list[6] == Token::Tcomma {
				let first: &[Token] = &tk_list[2..6];
				let second: &[Token] = &tk_list[7..(tk_list.len() - 1)];
				AST::AplusC(Box::new(parse(first)), Box::new(parse(second)))
			} else /* there are things in first and second param */ {
                let start_position = 2;
                let mut end_position = 2;
                for (index, token) in tk_list.iter().enumerate() {
                    if *token == Token::Tcomma {
                        if is_key_word(&tk_list[start_position..index]) {
                            end_position = index;
                            break;
                        }
                    }
                }

                if tk_list.len() < (end_position + 3) {
                    panic!("Invalid format of plusC.")
                } else if start_position == end_position || (end_position + 1) == (tk_list.len() - 1) {
                    panic!("Parsing Error Occurred!")
                } else {
                    let first: &[Token] = &tk_list[start_position..end_position];
                    let second: &[Token] = &tk_list[(end_position + 1) .. tk_list.len() - 1];
                    AST::AplusC(Box::new(parse(first)), Box::new(parse(second)))
                }
            }
		} /* [END] Token::TplusC */ ,

        Token::TmultC => {
            if tk_list.len() < 6 {
                panic!("Incorrect format of multC")
            }
			if tk_list[1] != Token::TleftParen {
				panic!("Missing Left Parenthesis.")
			}
			if tk_list[tk_list.len() - 1] != Token::TrightParen {
				panic!("Missing Right Parentesis.")
			}

            /* multC(x, y) */
			if tk_list.len() == 6 && tk_list[3] == Token::Tcomma {
                let temp = tk_list[2].clone();
				let first: &[Token] = &vec![temp];
                let temp = tk_list[4].clone();
				let second: &[Token] = &vec![temp];
				AST::AmultC(Box::new(parse(first)), Box::new(parse(second)))
			}

            /* multC(numC(x), numC(y)) */
            else if tk_list.len() == 12 && tk_list[6] == Token::Tcomma {
				let first: &[Token] = &tk_list[2..6];
				let second: &[Token] = &tk_list[7..(tk_list.len() - 1)];
				AST::AmultC(Box::new(parse(first)), Box::new(parse(second)))
			} else {
                let start_position = 2;
                let mut end_position = 2;
                for (index, token) in tk_list.iter().enumerate() {
                    if *token == Token::Tcomma {
                        if is_key_word(&tk_list[start_position..index]) {
                            end_position = index;
                            break;
                        }
                    }
                }
                if tk_list.len() < (end_position + 3) {
                    panic!("Invalid format of multC.")
                } else if start_position == end_position || (end_position + 1) == (tk_list.len() - 1) {
                    panic!("Parsing Error Occurred!")
                } else {
                    let first: &[Token] = &tk_list[start_position..end_position];
                    let second: &[Token] = &tk_list[(end_position + 1) .. tk_list.len() - 1];
                    AST::AmultC(Box::new(parse(first)), Box::new(parse(second)))
                }
            }
        } /* [END] Token::TmultC */ ,

        Token::TeqC => {
            if tk_list.len() < 6 {
                panic!("Incorrect format of eqC")
            }
			if tk_list[1] != Token::TleftParen {
				panic!("Missing Left Parenthesis.")
			}
			if tk_list[tk_list.len() - 1] != Token::TrightParen {
				panic!("Missing Right Parentesis.")
			}

            /* eqC(x, y) */
			if tk_list.len() == 6 && tk_list[3] == Token::Tcomma {
                let temp = tk_list[2].clone();
				let first: &[Token] = &vec![temp];
                let temp = tk_list[4].clone();
				let second: &[Token] = &vec![temp];
				AST::AeqC(Box::new(parse(first)), Box::new(parse(second)))
			} else if tk_list.len() == 12 && tk_list[6] == Token::Tcomma {
				let first: &[Token] = &tk_list[2..6];
				let second: &[Token] = &tk_list[7..(tk_list.len() - 1)];
				AST::AeqC(Box::new(parse(first)), Box::new(parse(second)))
			} else /* there are things in first and second param */ {
                let start_position = 2;
                let mut end_position = 2;
                for (index, token) in tk_list.iter().enumerate() {
                    if *token == Token::Tcomma {
                        if is_key_word(&tk_list[start_position..index]) {
                            end_position = index;
                            break;
                        }
                    }
                }
                if tk_list.len() < (end_position + 3) {
                    panic!("Invalid format of eqC.")
                } else if start_position == end_position || (end_position + 1) == (tk_list.len() - 1) {
                    panic!("Parsing Error Occurred!")
                } else {
                    let first: &[Token] = &tk_list[start_position..end_position];
                    let second: &[Token] = &tk_list[(end_position + 1) .. tk_list.len() - 1];
                    AST::AeqC(Box::new(parse(first)), Box::new(parse(second)))
                }
            }
		} /* [END] Token::TeqC */ ,

        Token::TifC => {
            if tk_list.len() < 8 {
                panic!("Incorrect format of ifC.")
            }
            if tk_list[1] != Token::TleftParen {
				panic!("Missing Left Parenthesis.")
			}
			if tk_list[tk_list.len() - 1] != Token::TrightParen {
				panic!("Missing Right Parenthesis.")
			}

            /* find the location of the first TyExprC */
            let first_start_pos = 2;
            let mut first_end_pos = 2;
            for (index, token) in tk_list.iter().enumerate() {
                if *token == Token::Tcomma {
                    if is_key_word(&tk_list[first_start_pos..index]) {
                        // println!("LOCATION OF FIRST COMMA: {}", index);
                        // println!("First Parameter: {:?}", &tk_list[first_start_pos..index]);
                        first_end_pos = index;
                        break;
                    }
                }
            }

            if tk_list.len() < (first_end_pos + 3) { panic!("Need More Argument for ifC.") }

            /* find the location of the second TyExprC */
            let second_start_pos = first_end_pos + 1;
            let mut second_end_pos = first_end_pos + 1;
            for (index, token) in tk_list.iter().enumerate() {

                // skip the first parameter
                if index < second_start_pos { continue; }

                if *token == Token::Tcomma {
                    if is_key_word(&tk_list[second_start_pos..index]) {
                        // println!("LOCATION OF SECOND COMMA: {}", index);
                        // println!("Second Parameter: {:?}", &tk_list[second_start_pos..index]);
                        second_end_pos = index;
                        break;
                    }
                }
            }

            // println!("Third Parameter: {:?}", &tk_list[(second_end_pos + 1) .. (tk_list.len() - 1)]);

            if tk_list.len() < (second_end_pos + 3) {
                panic!("Need More Argument for ifC.")
            } else if first_start_pos == first_end_pos || second_start_pos == second_end_pos {
                panic!("Parsing Error Occurred!")
            } else {
                let first: &[Token] = &tk_list[first_start_pos..first_end_pos];
                let second: &[Token] = &tk_list[second_start_pos..second_end_pos];
                let third: &[Token] = &tk_list[(second_end_pos + 1) .. (tk_list.len() - 1)];

                // println!("All together");
                // println!("FIRST {:?}", first);
                // println!("Second {:?}", second);
                // println!("Third {:?}", third);
                AST::AifC(Box::new(parse(first)), Box::new(parse(second)), Box::new(parse(third)))
            }
        } /* [END] Token::TifC */ ,

        Token::TappC => {
            if tk_list.len() < 6 {
                panic!("Incorrect format of appC")
            }
			if tk_list[1] != Token::TleftParen {
				panic!("Missing Left Parenthesis.")
			}
			if tk_list[tk_list.len() - 1] != Token::TrightParen {
				panic!("Missing Right Parentesis.")
			}

            /* appC(x, y) */
			if tk_list.len() == 6 && tk_list[3] == Token::Tcomma {
                let temp = tk_list[2].clone();
				let first: &[Token] = &vec![temp];
                let temp = tk_list[4].clone();
				let second: &[Token] = &vec![temp];
				AST::AappC(Box::new(parse(first)), Box::new(parse(second)))
			} else if tk_list.len() == 12 && tk_list[6] == Token::Tcomma {
				let first: &[Token] = &tk_list[2..6];
				let second: &[Token] = &tk_list[7..(tk_list.len() - 1)];
				AST::AappC(Box::new(parse(first)), Box::new(parse(second)))
			} else /* there are things in first and second param */ {
                let start_position = 2;
                let mut end_position = 2;
                for (index, token) in tk_list.iter().enumerate() {
                    if *token == Token::Tcomma {
                        if is_key_word(&tk_list[start_position..index]) {
                            end_position = index;
                            break;
                        }
                    }
                }

                if tk_list.len() < (end_position + 3) {
                    panic!("Invalid format of appC.")
                } else if start_position == end_position || (end_position + 1) == (tk_list.len() - 1) {
                    panic!("Parsing Error Occurred!")
                } else {
                    let first: &[Token] = &tk_list[start_position..end_position];
                    let second: &[Token] = &tk_list[(end_position + 1) .. tk_list.len() - 1];
                    AST::AappC(Box::new(parse(first)), Box::new(parse(second)))
                }
            }
		} /* [END] Token::TappC */ ,


        Token::TfdC => {
            if tk_list.len() < 10 {
                panic!("Incorrect format of fdC.")
            }
            if tk_list[1] != Token::TleftParen {
                panic!("Missing Left Parenthesis.")
            }
            if tk_list[tk_list.len() - 1] != Token::TrightParen {
                panic!("Missing Right Parenthesis.")
            }

            /* verify the first param is String type */
            if tk_list[2] != Token::Tquote || tk_list[4] != Token::Tquote || tk_list[5] != Token::Tcomma {
                panic!("First param must be String (Format Error)")
            }

            /* verify the second param is Type */
            if !is_type(&tk_list[6]) || tk_list[7] != Token::Tcomma {
                panic!("Second param must be Type (Format Error)")
            }

            /* verify the third param is Type */
            if !is_type(&tk_list[8]) || tk_list[9] != Token::Tcomma {
                panic!("Third param must be Type (Format Error)")
            }

            /* verify the forth param is TyExprC */
            if !is_key_word(&tk_list[10..(tk_list.len() - 1)]) {
                panic!("Forth param must be TyExprC")
            }

            let name = get_name(&tk_list[3].clone());

            let arg_type = get_type(&tk_list[6].clone());
            let ret_type = get_type(&tk_list[8].clone());

            // println!("arg type {:?}", arg_type);
            // println!("ret type {:?}", ret_type);
            let body: &[Token] = &tk_list[10..(tk_list.len() - 1)];
            AST::AfdC(name, Box::new(arg_type), Box::new(ret_type), Box::new(parse(body)))
        } /* [END] Token::TfdC */ ,

		Token::TrecC => {
			if tk_list.len() < 18 {
				panic!("Incorrect format of recC.")
			}
			if tk_list[1] != Token::TleftParen {
                panic!("Missing Left Parenthesis.")
            }
            if tk_list[tk_list.len() - 1] != Token::TrightParen {
                panic!("Missing Right Parenthesis.")
            }

			/* verify the first param is String type */
            if tk_list[2] != Token::Tquote || tk_list[4] != Token::Tquote || tk_list[5] != Token::Tcomma {
                panic!("First param must be String (Format Error)")
            }

			/* verify the second param is String type */
			if tk_list[6] != Token::Tquote || tk_list[8] != Token::Tquote || tk_list[9] != Token::Tcomma {
				panic!("Second param must be String (Format Error)")
			}

			/* verify the third param is Type */
            if !is_type(&tk_list[10]) || tk_list[11] != Token::Tcomma {
                panic!("Third param must be Type (Format Error)")
            }

			/* verify the forth param is Type */
            if !is_type(&tk_list[12]) || tk_list[13] != Token::Tcomma {
                panic!("Forth param must be Type (Format Error)")
            }

			/* verify the fifth param is TyExprC */
			let start_index = 14;
			let mut end_index = 14;
			for (index, token) in tk_list.iter().enumerate() {
				if index < start_index { continue; }
				if *token == Token::Tcomma {
					if is_key_word(&tk_list[start_index..index]) {
						end_index = index;
						break;
					}
				}
			}

			if tk_list.len() < (end_index + 3) {
				panic!("Need More Argument for recC.")
			}
			if start_index == end_index || (end_index + 1) == (tk_list.len() - 1) {
				panic!("Parsing Error Occurred!")
			}

			if !is_key_word(&tk_list[start_index..end_index]) {
				panic!("Fifth param must be TyExprC")
			}

			if !is_key_word(&tk_list[(end_index + 1) .. (tk_list.len() - 1)]) {
				panic!("Last param must be TyExprC")
			}

			let name1 = get_name(&tk_list[3].clone());
			let name2 = get_name(&tk_list[7].clone());
			let type1 = get_type(&tk_list[10].clone());
			let type2 = get_type(&tk_list[12].clone());
			let body1: &[Token] = &tk_list[start_index..end_index];
			let body2: &[Token] = &tk_list[(end_index + 1) .. (tk_list.len() - 1)];
			AST::ArecC(name1, name2, Box::new(type1), Box::new(type2), Box::new(parse(body1)), Box::new(parse(body2)))
		} /* [END] Token::TTrecC */ ,
	} /* [END] match statement */

} /* [END] parse function */

pub fn get_name(token: &Token) -> String {
    match token {
        Token::Tid(s) => s.to_string(),
        _ => "".to_string()
    }
}

pub fn get_type(token: &Token) -> Type {
    match token {
        Token::Ttype(t) => t.clone(),
        _ => panic!("Not a valid type")
    }
}

pub fn is_key_word(tk_list: &[Token]) -> bool {
    // println!("Parameter: {:?}", tk_list);
    let check_first = tk_list[0].clone();
    match check_first {
        Token::Tnum(_i) => {
            if tk_list.len() == 1 {
                true
            } else {
                false
            }
        },
        Token::Tid(_s) => {
            if tk_list.len() == 1 {
                true
            } else {
                false
            }
        },
        Token::Ttype(_t) => false,
        Token::TtrueC | Token::TfalseC => {
            if tk_list.len() == 1 {
                true
            } else {
                false
            }
        },
        Token::TleftParen | Token::TrightParen | Token::Tcomma | Token::Tquote => { false },
        Token::TnumC => {
            if tk_list.len() == 4 {
                true
            } else {
                false
            }
        },
        Token::TidC => {
            if tk_list.len() == 4 || tk_list.len() == 6 {
                true
            } else {
                false
            }
        }
        Token::TplusC | Token::TmultC | Token::TeqC | Token::TappC => {
            if tk_list.len() < 6 {
                false
            } else if tk_list[1] != Token::TleftParen {
				false
			} else if tk_list[tk_list.len() - 1] != Token::TrightParen {
				false
			} else {
                let start_index = 2;
                let mut end_index = 2;
                for (index, token) in tk_list.iter().enumerate() {
                    if *token == Token::Tcomma && is_key_word(&tk_list[start_index..index]){
                        end_index = index;
                        break;
                    }
                }
                if tk_list.len() < (end_index + 3) {
                    false
                } else if start_index == end_index || (end_index + 1) == (tk_list.len() - 1) {
                    false
                } else {
                    if !is_key_word(&tk_list[start_index..end_index]) {
                        false
                    } else if !is_key_word(&tk_list[(end_index + 1) .. (tk_list.len() - 1)]) {
                        false
                    } else {
                        true
                    }
                }
            }
        },
        Token::TifC => {
            if tk_list.len() < 8 {
                false
            } else if tk_list[1] != Token::TleftParen {
				false
			} else if tk_list[tk_list.len() - 1] != Token::TrightParen {
				false
			} else {
                let first_start_index = 2;
                let mut first_end_index = 2;
                for (index, token) in tk_list.iter().enumerate() {
                    if *token == Token::Tcomma && is_key_word(&tk_list[first_start_index..index]){
                        first_end_index = index;
                        break;
                    }
                }
                if tk_list.len() < (first_end_index + 3) {
                    false
                } else if first_start_index == first_end_index {
                    false
                } else {
                    let second_start_index = first_end_index + 1;
                    let mut second_end_index = first_end_index + 1;
                    for (index, token) in tk_list.iter().enumerate() {
                        if index < second_start_index { continue; }
                        if *token == Token::Tcomma && is_key_word(&tk_list[second_start_index..index]) {
                            second_end_index = index;
                            break;
                        }
                    }
                    if tk_list.len() < (second_end_index + 3) {
                        false
                    } else if second_start_index == second_end_index {
                        false
                    } else {
                        if !is_key_word(&tk_list[first_start_index..first_end_index]) {
                            false
                        } else if !is_key_word(&tk_list[second_start_index..second_end_index]) {
                            false
                        } else if !is_key_word(&tk_list[(second_end_index + 1) .. (tk_list.len() - 1)]) {
                            false
                        } else {
                            true
                        }
                    }
                }
            }
        } /* [END] ifC */ ,
        Token::TfdC => {
            if tk_list.len() < 10 {
				false
			} else if tk_list[1] != Token::TleftParen {
				false
			} else if tk_list[tk_list.len() - 1] != Token::TrightParen {
				false
			} else if tk_list[2] != Token::Tquote || tk_list[4] != Token::Tquote || tk_list[5] != Token::Tcomma {
				false
			} else if !is_type(&tk_list[6]) || tk_list[7] != Token::Tcomma {
				false
			} else if !is_type(&tk_list[8]) || tk_list[9] != Token::Tcomma {
				false
			} else if !is_key_word(&tk_list[10..(tk_list.len() - 1)]) {
				false
			} else {
				true
			}
        },
		Token::TrecC => {
			if tk_list.len() < 18 {
				false
			} else if tk_list[1] != Token::TleftParen {
				false
			} else if tk_list[tk_list.len() - 1] != Token::TrightParen {
				false
			} else if tk_list[2] != Token::Tquote || tk_list[4] != Token::Tquote || tk_list[5] != Token::Tcomma {
				false
			} else if tk_list[6] != Token::Tquote || tk_list[8] != Token::Tquote || tk_list[9] != Token::Tcomma {
				false
			} else if !is_type(&tk_list[10]) || tk_list[11] != Token::Tcomma {
				false
			} else if !is_type(&tk_list[12]) || tk_list[13] != Token::Tcomma {
				false
			} else {
                let start_index = 14;
                let mut end_index = 14;
                for (index, token) in tk_list.iter().enumerate() {
					if index < start_index { continue; }
                    if *token == Token::Tcomma && is_key_word(&tk_list[start_index..index]) {
                        end_index = index;
                        break;
                    }
                }
                if tk_list.len() < (end_index + 3) {
                    false
                } else if start_index == end_index || (end_index + 1) == (tk_list.len() - 1) {
                    false
                } else {
                    if !is_key_word(&tk_list[start_index..end_index]) {
                        false
                    } else if !is_key_word(&tk_list[(end_index + 1) .. (tk_list.len() - 1)]) {
                        false
                    } else {
                        true
                    }
                }
            }
		}
    }
} /* [END] is_key_word function */

/* ====================================================================================== */
/* ====================================================================================== */
/* ====================================================================================== */
/* ====================================================================================== */
/* ====================================================================================== */

/* --------------- Type Checkier Function ---------------
 * @parameter: an AST and a HashMap (type environment)
 * @return: a Type
 * Description: the function checks for the type.
 */
pub fn tc(ast: AST, tnv: &HashMap<String, Type>) -> Type {
	match ast {
		AST::AnumC(_i) => {Type::NumT},
		AST::AplusC(op1, op2) => {
			if tc(*op1, tnv) == Type::NumT && tc(*op2, tnv) == Type::NumT {
				Type::NumT
			} else {
				panic!("Invalid in plus!")
			}
		},
		AST::AmultC(op1, op2) => {
			if tc(*op1, &tnv) == Type::NumT && tc(*op2, &tnv) == Type::NumT {
				Type::NumT
			} else {
				panic!("Invalid in mult!")
			}
		},
		AST::AtrueC(_b) => Type::BoolT,
		AST::AfalseC(_b) => Type::BoolT,
		AST::AeqC(left, right) => {
			let left_type: Type = tc(*left, tnv);
			let right_type: Type = tc(*right, tnv);
			if left_type == right_type {
				Type::BoolT
			} else {
				panic!("Eq not matched!")
			}
		},
		AST::AifC(ifc, thenc, elsec) => {
			let if_type = tc(*ifc, tnv);
			if if_type != Type::BoolT {
				panic!("Invalid control flow!")
			} else {
				let then_type = tc(*thenc, tnv);
				let else_type = tc(*elsec, tnv);
				if then_type == else_type {
					then_type
				} else {
					panic!("If-else not matched!")
				}
			}
		},
		AST::AidC(s) => {
			ty_lookup(s, tnv)
		},
		AST::AfdC(name, inpt, ret, body) => {
			// println!("Recognized fun");
			let mut etnv: HashMap<String, Type> = tnv.clone();
			let einpt = inpt.clone();
			etnv.insert(name.to_string(), *einpt);
			// println!("Inserted var to einpt");
			check_tnv(&etnv);
			let body_type = tc(*body, &etnv);
			if body_type == *ret {
				Type::FunT(inpt, ret)
			} else {
				panic!("Function return not matched!")
			}
		},
		AST::ArecC(name1, name2, type1, type2, body1, body2) => {
			let mut etnv: HashMap<String, Type> = tnv.clone();
			let e_type1  = type1.clone();
			let e_type2 = type2.clone();
			let e_type22 = type2.clone();
			etnv.insert(name1.to_string(), Type::FunT(Box::new(*e_type1), Box::new(*e_type2)));
			check_tnv(&etnv);
			let mut etnv2: HashMap<String, Type> = etnv.clone();
			etnv2.insert(name2.to_string(), *e_type22);
			let body_type = tc(*body1, &etnv2);
			if body_type == *type2 {
				tc(*body2, &etnv)
			} else {
				panic!("Function return not matched!")
			}
		},
		AST::AappC(fun, arg) => {
			let arg_type = tc(*arg, tnv);
			let fun_type = tc(*fun, tnv);
			match fun_type {
				Type::FunT(inpt_type, ret_type) => {
					if *inpt_type == arg_type {
						*ret_type
					} else {
						panic!("Function argument not matched!")
					}
				}
				Type::NumT => panic!("Not a function!"),
				Type::BoolT => panic!("Not a function!"),
			}
		},
		/* these cases will not happen */
        AST::None => Type::BoolT,
		AST::AtypeC(_t) => Type::BoolT,
	}
}

pub fn ty_lookup (id: String, tnv: &HashMap<String, Type>) -> Type {
	match tnv.get(&id) {
		Some(t) => t.clone(),
		_ => panic!("Unexpected identifier: {}", &id),
	}
}

pub fn check_tnv(tnv: &HashMap<String, Type>) {
	// println!("check tnv");
    let mut id = String::new();
	for (var, _ty) in tnv.iter() {
    	// println!("found {}", var);
        id = var.to_string();
	}
	ty_lookup(id.to_string(), &tnv);
}

#[cfg(test)]
mod tests {

	use super::*;

    #[test]
    fn num_test() {
        let input = String::from("numC(5)");
		let parser: Vec<Token> = tokenize(input);
	    let ast = parse(&parser);
	    let tnv: HashMap<String, Type> = HashMap::new();
	    let result = tc(ast, &tnv);
		assert_eq!(result, Type::NumT)
    }

	#[test]
	fn plus_test() {
        let input = String::from("plusC(numC(3), numC(5))");
		let parser: Vec<Token> = tokenize(input);
	    let ast = parse(&parser);
	    let tnv: HashMap<String, Type> = HashMap::new();
	    let result = tc(ast, &tnv);
		assert_eq!(result, Type::NumT)
    }

	#[test]
	fn mult_test() {
        let input = String::from("multC(numC(10), numC(20))");
		let parser: Vec<Token> = tokenize(input);
	    let ast = parse(&parser);
	    let tnv: HashMap<String, Type> = HashMap::new();
	    let result = tc(ast, &tnv);
		assert_eq!(result, Type::NumT)
    }

	#[test]
	fn true_test() {
        let input = String::from("trueC");
		let parser: Vec<Token> = tokenize(input);
	    let ast = parse(&parser);
	    let tnv: HashMap<String, Type> = HashMap::new();
	    let result = tc(ast, &tnv);
		assert_eq!(result, Type::BoolT)
    }

	#[test]
	fn false_test() {
        let input = String::from("falseC");
		let parser: Vec<Token> = tokenize(input);
	    let ast = parse(&parser);
	    let tnv: HashMap<String, Type> = HashMap::new();
	    let result = tc(ast, &tnv);
		assert_eq!(result, Type::BoolT)
    }

	#[test]
	fn eq_test() {
		// eqC(plusC(4, 6), multC(2, 5))
        let input = String::from("eqC(plusC(numC(4), numC(6)), multC(numC(2), numC(5)))");
		let parser: Vec<Token> = tokenize(input);
	    let ast = parse(&parser);
	    let tnv: HashMap<String, Type> = HashMap::new();
	    let result = tc(ast, &tnv);
		assert_eq!(result, Type::BoolT)
    }

	#[test]
	fn if_test() {
		// provided by Piazza @37
        let input = String::from(r#"ifC(eqC(appC(fdC("n", numT, numT, plusC(numC(100), idC("n"))), numC(100)), numC(100)), appC(fdC("i", numT, boolT, eqC(idC("i"), numC(5))), numC(20)), falseC)"#);
		let parser: Vec<Token> = tokenize(input);
	    let ast = parse(&parser);
	    let tnv: HashMap<String, Type> = HashMap::new();
	    let result = tc(ast, &tnv);
		assert_eq!(result, Type::BoolT)
    }

	#[test]
	#[should_panic]
	fn id_test() {
        let input = String::from(r#"idC("n")"#);
		let parser: Vec<Token> = tokenize(input);
	    let ast = parse(&parser);
	    let tnv: HashMap<String, Type> = HashMap::new();
	    let result = tc(ast, &tnv);
		assert_eq!(result, Type::BoolT)
    }

	#[test]
	fn app_test() {
		// provided by Piazza @37
        let input = String::from(r#"appC(fdC("n", numT, numT, multC(numC(3), idC("n"))), numC(5))"#);
		let parser: Vec<Token> = tokenize(input);
	    let ast = parse(&parser);
	    let tnv: HashMap<String, Type> = HashMap::new();
	    let result = tc(ast, &tnv);
		assert_eq!(result, Type::NumT)
    }

	#[test]
	fn fd_test() {
		// provided by Piazza @37
        let input = String::from(r#"fdC("n", numT, numT, numC(52))"#);
		let parser: Vec<Token> = tokenize(input);
	    let ast = parse(&parser);
	    let tnv: HashMap<String, Type> = HashMap::new();
	    let result = tc(ast, &tnv);
		assert_eq!(result, Type::FunT(Box::new(Type::NumT), Box::new(Type::NumT)))
    }

	#[test]
	fn rec_test() {
		// provided by Piazza @37
        let input = String::from(r#"recC("n", "x", numT, numT, plusC(1, 2), plusC(3, 4))"#);
		let parser: Vec<Token> = tokenize(input);
	    let ast = parse(&parser);
	    let tnv: HashMap<String, Type> = HashMap::new();
	    let result = tc(ast, &tnv);
		assert_eq!(result, Type::NumT)
    }
}
