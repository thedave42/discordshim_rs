use crate::{Context, Error};


#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn status(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    println!("received /status {}", ctx.data().channel_id);
    let serenity_ctx = ctx.serenity_context().clone();
    ctx.data().server
        .read()
        .await
        .send_stats(ctx.data().channel_id, serenity_ctx)
        .await;
    Ok(())
}
