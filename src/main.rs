use clap::{Parser, ValueEnum} ;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE
}

#[derive(Parser, Debug)]
#[command(author="Sam Sargent", version, about="Send multiple requests to an endpoint", long_about = None)]
struct Cli {
    #[arg(short='u', long="url")]
    url: String,

    #[arg(short='m', long="method", value_enum)]
    method: HttpMethod,

    #[arg(short='x', long="amount", default_value_t = 1)]
    amount: u8,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let mut handlers: Vec<tokio::task::JoinHandle<Result<reqwest::Response, reqwest::Error>>> = Vec::with_capacity(cli.amount as usize);

    for _num in 1..=cli.amount {

        let url = cli.url.to_owned();

        let task = tokio::spawn(async move {
            match cli.method {
                HttpMethod::GET => {
                    reqwest::get(url).await
                },
                _ => { todo!("Only GET supported") }
            }
        });

        handlers.push(task);
    };

    for handler in handlers {
        let res = handler.await.unwrap().unwrap().text().await.unwrap();
        println!("{}", res);
    }

}

