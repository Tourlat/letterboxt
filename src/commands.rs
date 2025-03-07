use crate::{Context, Error};
use crate::film_scraper::{extract_film_meta_datas, extract_film_meta_datas_from_url};
use crate::film_error::FilmError;
use crate::utils::is_url;

async fn handle_film_error(ctx: &Context<'_>, error: FilmError) -> Result<(), Error> {
    match error {
        FilmError::ParseError => {
            ctx.say("Error: Failed to parse the film data.").await?;
        }
        FilmError::SelectorError => {
            ctx.say("Error: Failed to select the required elements from the film data.").await?;
        }
        FilmError::AttributeError => {
            ctx.say("Error: Failed to extract the required attributes from the film data.").await?;
        }
        FilmError::ReqwestError(e) => {
            ctx.say(format!("Error: Network request failed with error: {}", e)).await?;
        }
        FilmError::NotFoundError => {
            ctx.say("Error: Film not found.").await?;
        }
    }
    Ok(())
}

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help information for the usage" ]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "Help section",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn rate(ctx: Context<'_>, 
#[description = "Find the rating of a film"]
#[autocomplete = "poise::builtins::autocomplete_command"] 
film_name: String,
) -> Result<(), Error> {
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
pub async fn release_year(ctx: Context<'_>, 
#[description = "Find the release year a film"]
#[autocomplete = "poise::builtins::autocomplete_command"] 
film_name: String) -> Result<(), Error> {
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
pub async fn director(ctx: Context<'_>,
#[description = "Find the director of a film"]
#[autocomplete = "poise::builtins::autocomplete_command"] 
 film_name: String) -> Result<(), Error> {
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
pub async fn synopsis(ctx: Context<'_>,
#[description = "Find the synopsis of a film"]
#[autocomplete = "poise::builtins::autocomplete_command"] 
film_name: String) -> Result<(), Error> {
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
pub async fn genres(ctx: Context<'_>,
#[description = "Find the different genres of a film (action, adventure...)"]
#[autocomplete = "poise::builtins::autocomplete_command"]  
film_name: String) -> Result<(), Error> {
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
pub async fn all(ctx: Context<'_>,
#[description = "Find multiple information about a film (rating, release year, director, synopsis, genres)"]
#[autocomplete = "poise::builtins::autocomplete_command"] 
 film_param: String) -> Result<(), Error> {
    let metadata_result = if is_url(&film_param) {
        extract_film_meta_datas_from_url(&film_param).await
    } else {
        extract_film_meta_datas(&film_param).await
    };
    match metadata_result {
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