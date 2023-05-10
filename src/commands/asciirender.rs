use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::{
        ApplicationCommandInteraction, CommandDataOptionValue,
    },
};

pub async fn run(commands: &ApplicationCommandInteraction) -> String {
    // knowing what this does helps a lot
    let az = commands
        .data
        .options
        .get(0)
        .expect("expected valid azimuth")
        .resolved
        .as_ref()
        .expect("expected user object");
    let al = commands
        .data
        .options
        .get(1)
        .expect("expected valid altitude")
        .resolved
        .as_ref()
        .expect("expected user object");

    // convert from type "CommandDataOptionValue" to an f64
    let az_f64: f64 = match az {
        CommandDataOptionValue::Number(n) => *n,
        _ => panic!("no f64 given!"),
    };
    let al_f64: f64 = match al {
        CommandDataOptionValue::Number(n) => *n,
        _ => panic!("no f64 given!"),
    };

    // handle command execution
    // might use https://crates.io/crates/ascii_renderer
    let render_cube = command_runner(az_f64, al_f64);
    dbg!(String::from_utf8(render_cube.stderr).expect("utf8")); // brain completely exploded
    let output = String::from_utf8(render_cube.stdout).expect("invalid utf8");
    // use code block to make the cube not look terrible due to the discord font.
    let output2 = format!("{}", "`".to_string() + &output + "`");
    //  let channel_id = commands.channel_id; // lmao, tried using "Interaction" struct the entire time
    // regex(output)
    output2
}

fn command_runner(az_f64: f64, al_f64: f64) -> std::process::Output {
    let render_cube = std::process::Command::new("./render/3d-ascii-viewer")
        .arg("./render/cube2.obj")
        .arg("-w")
        .arg("44")
        .arg("-h")
        .arg("44")
        .arg("--snap")
        .arg(az_f64.to_string())
        .arg(al_f64.to_string())
        .output()
        .expect("failed to start renderer.");
    render_cube
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("render")
        .description("render one of the given models with the given angles")
        .create_option(|option| {
            option
                .name("azimuth")
                .description("azimuth of the 3d model")
                .kind(serenity::model::prelude::command::CommandOptionType::Number)
                .min_number_value(-360.0)
                .max_number_value(360.0)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("altitude")
                .description("altitude of the 3d model")
                .kind(serenity::model::prelude::command::CommandOptionType::Number)
                .min_number_value(-360.0)
                .max_number_value(360.0)
                .required(true)
        })
}
