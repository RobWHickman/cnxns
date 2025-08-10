use crate::app::data_types::Player;
use axum::response::Html;

pub async fn home_page(players: Vec<Player>) -> Html<String> {
    let player1_name = &players[0].player_name;
    let player2_name = &players[1].player_name;

    let html_string = format!(
        r#"
       <!DOCTYPE html>
       <html>
       <head>
           <title>Tifo Podcast Football Connections Game</title>
           <style>
               body {{ 
                   font-family: "Helvetica Neue", Helvetica, Arial, sans-serif; 
font-weight: 900; 
                   margin: 0; 
                   padding: 50px;
                   background-color: #f8d763;
                   min-height: 100vh;
                   position: relative;
               }}
               .game-container {{ max-width: 400px; margin: 0 auto; }}
               .player-box {{ 
                   border: 1px solid #ddd; 
                   padding: 15px; 
                   margin: 5px 0; 
                   text-align: center; 
                   background: linear-gradient(180deg, rgba(255,255,255,0.9) 0%, rgba(245,245,245,0.9) 100%);
                   border-radius: 12px;
                   box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                   height: 50px;
                   width: 100%;
                   box-sizing: border-box;
                   display: flex;
                   align-items: center;
                   justify-content: center;
               }}
               .connection-input {{ 
                   width: 100%; 
                   padding: 15px; 
                   margin: 10px 0; 
                   border: 1px solid #ddd; 
                   border-radius: 12px;
                   font-size: 16px;
                   box-sizing: border-box;
                   height: 50px;
                   box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                   background-color: rgba(255,255,255,0.9);
               }}
                .corner-images {{
                    position: fixed;
                    bottom: 200px;
                    width: 100%;
                    pointer-events: none;
                }}
                .corner-img {{
                    position: absolute;
                    height: 100px;
                    width: auto;
                }}
                .corner-img.left {{
                    left: -30px;
                }}
                .corner-img.right {{
                    right: 70px;
                }}
                   .bottom-stripes {{
                   position: fixed;
                   bottom: 30px;
                   left: 0;
                   width: 100%;
                   height: 50px;
               }}
               .stripe {{
                   width: 100%;
                   height: 22px;
                   background-color: #000;
                   margin-bottom: 6px;
               }}
           </style>
       </head>
       <body>
           <div class="game-container">
               <h1>Tifo Podcast Football Connections Game</h1>
               
               <div class="player-box">
                   <h3>{}</h3>
               </div>
               
               <input type="text" class="connection-input" placeholder="Add a connecting player...">
               
               <div class="player-box">
                   <h3>{}</h3>
               </div>
               
               <p>Connect these players through their teammates!</p>
           </div>
           
           <div class="corner-images">
               <img src="/static/img/tifo.png" alt="Tifo" class="corner-img left">
               <img src="/static/img/fbref.svg" alt="FBRef" class="corner-img right">
           </div>
           
           <div class="bottom-stripes">
               <div class="stripe"></div>
               <div class="stripe"></div>
           </div>
       </body>
       </html>
       "#,
        player1_name, player2_name
    );

    Html(html_string)
}
