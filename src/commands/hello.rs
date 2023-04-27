use rand::Rng;
use serenity::builder::CreateApplicationCommand;

pub async fn run() -> String {
    let greetings = [
        "Hallo",
        "Hi",
        "Salut",
        "¿Qué tal?",
        "привіт",
        "Nǐ hǎo",
        "Salve",
        "Konnichiwa",
        "Oi",
        "Anyoung",
        "Asalaam alaikum",
        "Halløj",
        "Hujambo",
        "Hoi",
        "Yassou",
        "Dzień dobry",
        "Selamat siang",
        "Namaste",
        "Selam",
        "Shalom",
        "Tjena",
        "Hei",
    ];

    let mut rng  = rand::thread_rng();
    let rnum: usize = rng.gen_range(0..21); // BE VERY CAREFUL TO USE CORRECT ARRAY LEN
    let selection = greetings[rnum].to_string();
    selection
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("hello").description("hallo sagen jtz")
}
