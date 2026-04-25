use anchor_lang::prelude::*; 

declare_id!("FizEkwtYPT437J4Wqm19bDRpEkc2TBfk1eEuKCYsu5Zq");

 #[program] 
 pub mod mi_proyecto_cine {
   use super::*;

pub fn crear_perfil_cine(ctx: Context<CrearPerfil>, nombre_usuario: String) -> Result<()> {
   let perfil = &mut ctx.accounts.perfil_datos;
   require!(nombre_usuario.len() <= 20, ErrorCode::NombreMuyLargo);
   perfil.nombre_usuario = nombre_usuario;
   perfil.mis_peliculas = Vec::new();
   Ok(())
}

pub fn agregar_pelicula(ctx: Context<ActualizarPerfil>, titulo: String, genero: String, calificacion: u8) -> Result<()> {
   let perfil_datos = &mut ctx.accounts.perfil_datos;
   let nueva_pelicula = Pelicula { titulo, genero, calificacion };
   perfil_datos.mis_peliculas.push(nueva_pelicula);
   Ok(())
} 


 pub fn editar_pelicula(ctx: Context<ActualizarPerfil>, titulo_buscar: String, nuevo_genero: String, nueva_califa: u8) -> Result<()> {
   let perfil_datos = &mut ctx.accounts.perfil_datos;

   
    if let Some(pos) = perfil_datos.mis_peliculas.iter().position(|p| p.titulo == titulo_buscar) {
       perfil_datos.mis_peliculas[pos].genero = nuevo_genero;
       perfil_datos.mis_peliculas[pos].calificacion = nueva_califa; 
       msg!("¡Película '{}' actualizada correctamente!", titulo_buscar);
   } else {
       return
       Err(ErrorCode::PeliculaNoEncontrada.into());
   } Ok(())
  } 
} 

#[derive(Accounts)]
pub struct CrearPerfil<'info> {
     #[account(init, payer = usuario, space = 8 + 40 + 600)]
     pub perfil_datos: Account<'info, PerfilUsuario>,
     #[account(mut)]
     pub usuario: Signer<'info>,
     pub system_program: Program<'info, System>,
} 

#[derive(Accounts)]
 pub struct ActualizarPerfil<'info> { 
    #[account(mut)]
    pub perfil_datos: Account<'info, PerfilUsuario>, 
}

#[account]
pub struct PerfilUsuario {
    pub nombre_usuario: String,
    pub mis_peliculas: Vec<Pelicula>, 
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Pelicula { 
    pub titulo: String, 
    pub genero: String, 
    pub calificacion: u8,
}
#[error_code] 
pub enum ErrorCode {
   #[msg("El nombre de usuario es demasiado largo.")]
   NombreMuyLargo,
   #[msg("No se encontró la película con ese título.")]
   PeliculaNoEncontrada, 
}
