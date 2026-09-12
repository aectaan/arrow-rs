// This code is generated so we don't want to fix any lint violations manually.
#[allow(clippy::allow_attributes)]
#[allow(clippy::all)]
mod generated {
    #![allow(missing_docs)]
    include!("arrow.flight.protocol.sql.rs");
}

pub mod metadata;

#[cfg(test)]
mod tests {

    use buffa_types::Any;

    use crate::sql::generated::{CommandStatementQuery, TicketStatementQuery};

    #[test]
    fn test_type_url() {
        assert_eq!(
            TicketStatementQuery::TYPE_URL,
            "type.googleapis.com/arrow.flight.protocol.sql.TicketStatementQuery"
        );
        assert_eq!(
            CommandStatementQuery::TYPE_URL,
            "type.googleapis.com/arrow.flight.protocol.sql.CommandStatementQuery"
        );
    }

    #[test]
    fn test_buffa_any_pack_unpack() {
        let query = CommandStatementQuery {
            query: "select 1".to_string(),
            transaction_id: None,
        };
        let any = Any::pack(&query, CommandStatementQuery::TYPE_URL);
        assert!(any.is_type(CommandStatementQuery::TYPE_URL));
        let unpack_query = any
            .unpack_if(CommandStatementQuery::TYPE_URL)
            .unwrap()
            .unwrap();
        assert_eq!(query, unpack_query);
    }
}
