FROM scratch

ENV DELIMITER=',' \
    FILES="" \
    REPLACE_DELIMITER='=' \
    REPLACE="foo=bar"

ADD target/x86_64-unknown-linux-musl/release/regex-replacer /

WORKDIR /app

CMD ["/regex-replacer"]
