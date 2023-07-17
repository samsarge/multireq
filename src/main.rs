use clap::{Parser, ValueEnum} ;

use hyper::Client;
use hyper_tls::HttpsConnector;


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

    let mut handlers: Vec<tokio::task::JoinHandle<Result<hyper::Response<hyper::Body>, hyper::Error>>> = Vec::with_capacity(cli.amount as usize);


    for _num in 1..=cli.amount {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let uri = cli.url.parse().unwrap();

        let task = tokio::spawn(async move {
            
            match cli.method {
                HttpMethod::GET => {
                    client.get(uri).await
                },
                _ => { todo!("Only GET supported") }
            }
        });

        handlers.push(task);
    };

    for handler in handlers {
        let res = handler.await.unwrap().unwrap();
        // These are streams, we'll need to read them properly if we want
        // to display response bodies.
        // while let Some(next) = res.data().await {
        //     let chunk = next?;
        //     io::stdout().write_all(&chunk).await?;
        // }
        println!("{:?}", res.body());
    }

}

