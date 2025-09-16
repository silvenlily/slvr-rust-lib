/****************************************
*  NOTICE
*  ______________________________________
*
*  Copyright 2023 Tony Nguyen
*
*  Project:
*      Name: to_snake_case
*      Links:
*         - https://gitlab.com/t101/to_snake_case
*
*      Modified: 20-Feb-2023
*  ______________________________________
*
*  Licensed under the Apache License, Version 2.0 (the "License");
*  you may not use this file except in compliance with the License.
*  You may obtain a copy of the License at
*
*      http://www.apache.org/licenses/LICENSE-2.0
*
*  Unless required by applicable law or agreed to in writing, software
*  distributed under the License is distributed on an "AS IS" BASIS,
*  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*  See the License for the specific language governing permissions and
*  limitations under the License.
*
******************************************/

pub trait ToSnakeCase : AsRef<str> {
    fn to_snake_case(&self) -> String;
}

impl <T> ToSnakeCase for T where T: AsRef<str> {
    fn to_snake_case(&self) -> String {
        let text = self.as_ref();

        let mut buffer = String::with_capacity(text.len() + text.len()/2);

        let mut text = text.chars();

        if let Some(first) = text.next() {
            let mut n2: Option<(bool, char)> = None;
            let mut n1: (bool, char) = (first.is_lowercase(), first);

            for c in text {
                let prev_n1 = n1.clone();

                let n3 = n2;
                n2 = Some(n1);
                n1 = (c.is_lowercase(), c);

                // insert underscore if acronym at beginning
                // ABc -> a_bc
                if let Some((false, c3)) = n3
                    && let Some((false, c2)) = n2
                    && n1.0
                    && c3.is_uppercase()
                    && c2.is_uppercase() {
                    buffer.push('_');
                }

                buffer.push_str(&prev_n1.1.to_lowercase().to_string());

                // insert underscore before next word
                // abC -> ab_c
                if let Some((true, _)) = n2 && n1.1.is_uppercase() {
                    buffer.push('_');
                }
            }

            buffer.push_str(&n1.1.to_lowercase().to_string());
        }

        buffer
    }
}
