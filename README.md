# serde_json_path_to_error

why

- You get data from external service you don't control
- Data contians PII that mustn't be logged
- You are already using `serde_json` extensively

why not

- you've benched and de/serializtion is the bottleneck
- you only want to use path_to_error in a few places