#[cfg(feature = "authorization")]
#[cfg(test)]
mod authorization_tests {
    use std::io;

    use async_stream_http_tunnel_grader::Authorization;

    #[test]
    fn basic() -> io::Result<()> {
        let authorization = Authorization::basic("aladdin".to_owned(), "opensesame".to_owned());

        assert_eq!(
            authorization.header_value(),
            "Basic YWxhZGRpbjpvcGVuc2VzYW1l"
        );

        Ok(())
    }

    #[test]
    fn bearer() -> io::Result<()> {
        let authorization = Authorization::bearer("mF_9.B5f-4.1JqM".to_owned());

        assert_eq!(authorization.header_value(), "Bearer mF_9.B5f-4.1JqM");

        Ok(())
    }
}
