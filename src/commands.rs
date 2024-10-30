use crate::{Context, Error};
use crate::film_scraper::extract_film_meta_datas;
use crate::film_error::FilmError;

async fn handle_film_error(ctx: &Context<'_>, error: FilmError) -> Result<(), Error> {
    match error {
        FilmError::ParseError | FilmError::SelectorError | FilmError::AttributeError | FilmError::ReqwestError(_) => {
            ctx.say("Film not found").await?;
        }
    }
    Ok(())
}

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn rate(ctx: Context<'_>, film_name: String) -> Result<(), Error> {
    match extract_film_meta_datas(&film_name).await {
        Ok(metadata) => {
            ctx.say(format!("Rating: {:.2}", metadata.rating)).await?;
        }
        Err(e) => {
            handle_film_error(&ctx, e).await?;
        }
    }
    Ok(())
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn release_year(ctx: Context<'_>, film_name: String) -> Result<(), Error> {
    match extract_film_meta_datas(&film_name).await {
        Ok(metadata) => {
            ctx.say(format!("Release Year: {}", metadata.release_year)).await?;
        }
        Err(e) => {
            handle_film_error(&ctx, e).await?;
        }
    }
    Ok(())
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn director(ctx: Context<'_>, film_name: String) -> Result<(), Error> {
    match extract_film_meta_datas(&film_name).await {
        Ok(metadata) => {
            ctx.say(format!("Director: {}", metadata.director)).await?;
        }
        Err(e) => {
            handle_film_error(&ctx, e).await?;
        }
    }
    Ok(())
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn synopsis(ctx: Context<'_>, film_name: String) -> Result<(), Error> {
    match extract_film_meta_datas(&film_name).await {
        Ok(metadata) => {
            ctx.say(format!("Synopsis: {}", metadata.synopsis)).await?;
        }
        Err(e) => {
            handle_film_error(&ctx, e).await?;
        }
    }
    Ok(())
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn genres(ctx: Context<'_>, film_name: String) -> Result<(), Error> {
    match extract_film_meta_datas(&film_name).await {
        Ok(metadata) => {
            ctx.say(format!("Genres: {}", metadata.genres.join(", "))).await?;
        }
        Err(e) => {
            handle_film_error(&ctx, e).await?;
        }
    }
    Ok(())
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn all(ctx: Context<'_>, film_name: String) -> Result<(), Error> {
    match extract_film_meta_datas(&film_name).await {
        Ok(metadata) => {
            ctx.say(format!("Rating: {:.2}\nRelease Year: {}\nDirector: {}\nSynopsis: {}\nGenres: {}",
                metadata.rating,
                metadata.release_year,
                metadata.director,
                metadata.synopsis,
                metadata.genres.join(", ")
            )).await?;
        }
        Err(e) => {
            handle_film_error(&ctx, e).await?;
        }
    }
    Ok(())
}