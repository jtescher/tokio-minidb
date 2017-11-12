use std::str;
use nom::{multispace, IResult};

use DBCommand;

pub fn parse_command(input: &[u8]) -> DBCommand {
    match nom_parse(input) {
        IResult::Done(_, command) => command,
        _ => DBCommand::Bad,
    }
}

// Helper macro to extract word [u8] -> word str
named!(word<&str>, map_res!(is_not!(" \t\r\n"), str::from_utf8));

// Helper macro to extract rest of the input to str
named!(rest_str<&str>, map_res!(is_not!("\r\n"), str::from_utf8));

// Parse commands
named!(nom_parse<&[u8], DBCommand>,
    alt!(cmd_get | cmd_put | cmd_delete)
);

// Get by key command
named!(cmd_get<&[u8], DBCommand>,
    do_parse!(
        tag_no_case!("get") >>
        multispace          >>
        key: word           >>
        (DBCommand::Get(key.to_string()))
    )
);

// Put value for key command
named!(cmd_put<&[u8], DBCommand>,
    do_parse!(
        tag_no_case!("put") >>
        multispace          >>
        key: word           >>
        multispace          >>
        value: rest_str     >>
        (DBCommand::Put(key.to_string(), value.to_string()))
    )
);

// Delete by key command
named!(cmd_delete<&[u8], DBCommand>,
    do_parse!(
        tag_no_case!("delete") >>
        multispace             >>
        key: word              >>
        (DBCommand::Delete(key.to_string()))
    )
);
