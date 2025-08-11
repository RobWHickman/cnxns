use crate::app::data_types::Player;
use axum::response::Html;

pub async fn home_page(players: Vec<Player>) -> Html<String> {
    let player1_name = &players[0].player_name;
    let player2_name = &players[1].player_name;

    let html = format!(
        r#"<!DOCTYPE html>
        <html>
        <head>
            <title>Tifo Podcast Football Connections Game</title>
            {}
        </head>
        <body>
            {}
            {}
            {}
        </body>
        </html>"#,
        get_styles(),
        get_game_content(player1_name, player2_name),
        get_footer_images(),
        get_javascript()
    );

    Html(html)
}

fn get_styles() -> &'static str {
    r#"<style>
        body { 
            font-family: "Helvetica Neue", Helvetica, Arial, sans-serif; 
            font-weight: 900; 
            margin: 0; 
            padding: 50px;
            background-color: #f8d763;
            min-height: 100vh;
            position: relative;
        }
        .game-container { max-width: 400px; margin: 0 auto; position: relative; }
        .player-box { 
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
        }
        .input-container {
            position: relative;
        }
        .connection-input { 
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
        }
        .autocomplete-dropdown {
            position: absolute;
            top: 100%;
            left: 0;
            right: 0;
            background: white;
            border: 1px solid #ddd;
            border-radius: 8px;
            max-height: 200px;
            overflow-y: auto;
            z-index: 1000;
            display: none;
        }
        .autocomplete-item {
            padding: 10px;
            cursor: pointer;
            border-bottom: 1px solid #eee;
        }
        .autocomplete-item:hover {
            background-color: #f5f5f5;
        }
        .corner-images {
            position: fixed;
            bottom: 200px;
            width: 100%;
            pointer-events: none;
        }
        .corner-img {
            position: absolute;
            height: 100px;
            width: auto;
        }
        .corner-img.left {
            left: -30px;
        }
        .corner-img.right {
            right: 70px;
        }
        .bottom-stripes {
            position: fixed;
            bottom: 30px;
            left: 0;
            width: 100%;
            height: 50px;
        }
        .stripe {
            width: 100%;
            height: 22px;
            background-color: #000;
            margin-bottom: 6px;
        }
    </style>"#
}

fn get_game_content(player1_name: &str, player2_name: &str) -> String {
    format!(
        r#"<div class="game-container">
            <h1>Tifo Podcast Football Connections Game</h1>
            
            <div class="player-box">
                <h3>{}</h3>
            </div>
            
            <div class="input-container">
                <input type="text" id="player-search" class="connection-input" placeholder="Add a connecting player...">
                <div id="autocomplete-dropdown" class="autocomplete-dropdown"></div>
            </div>
            
            <div class="player-box">
                <h3>{}</h3>
            </div>
            
            <p>Connect these players through their teammates!</p>
        </div>"#,
        player1_name, player2_name
    )
}

fn get_footer_images() -> &'static str {
    r#"<div class="corner-images">
        <img src="/static/img/tifo.png" alt="Tifo" class="corner-img left">
        <img src="/static/img/fbref.svg" alt="FBRef" class="corner-img right">
    </div>
    
    <div class="bottom-stripes">
        <div class="stripe"></div>
        <div class="stripe"></div>
    </div>"#
}

fn get_javascript() -> &'static str {
    r#"<script>
       const searchInput = document.getElementById('player-search');
       const dropdown = document.getElementById('autocomplete-dropdown');

       let searchTimeout;

       searchInput.addEventListener('input', function() {
           const query = this.value.trim();
           
           if (query.length < 2) {
               dropdown.style.display = 'none';
               return;
           }

           clearTimeout(searchTimeout);
           searchTimeout = setTimeout(() => {
               fetch(`/api/search?q=${encodeURIComponent(query)}`)
                   .then(response => response.json())
                   .then(players => {
                       dropdown.innerHTML = '';
                       
                       if (players.length === 0) {
                           dropdown.style.display = 'none';
                           return;
                       }

                       players.forEach(player => {
                           const item = document.createElement('div');
                           item.className = 'autocomplete-item';
                           item.textContent = player.player_name;
                           item.addEventListener('click', () => {
                               searchInput.value = player.player_name;
                               dropdown.style.display = 'none';
                           
                               checkPlayerConnection(player.player_id);
                           });
                           dropdown.appendChild(item);
                       });

                       dropdown.style.display = 'block';
                   })
                   .catch(error => {
                       console.error('Search error:', error);
                       dropdown.style.display = 'none';
                   });
           }, 300);
       });

       function checkPlayerConnection(selectedPlayerId) {
           const player1_id = "c0c5ee74";
           
           fetch('/api/check-connection', {
               method: 'POST',
               headers: { 'Content-Type': 'application/json' },
               body: JSON.stringify({ 
                   player1_id: player1_id, 
                   player2_id: selectedPlayerId 
               })
           })
           .then(response => response.json())
           .then(data => {
               console.log('Received data:', data);
               if (!data.success) {
                   console.log('About to show alert');
                   alert('No shared matches!');
               } else {
                   lockInPlayer(selectedPlayerId);
               }
           })
           .catch(error => console.error('Connection check error:', error));
       }

       function lockInPlayer(playerId) {
           const currentInput = document.getElementById('player-search');
           const container = currentInput.parentElement;
           
           // Disable current input
           currentInput.disabled = true;
           currentInput.style.backgroundColor = '#e9e9e9';
           
           // Create new input container
           const newInputContainer = document.createElement('div');
           newInputContainer.className = 'input-container';
           newInputContainer.innerHTML = `
               <input type="text" class="connection-input" placeholder="Add another connecting player...">
               <div class="autocomplete-dropdown"></div>
           `;
           
           // Insert after current container
           container.parentNode.insertBefore(newInputContainer, container.nextSibling);
       }

       document.addEventListener('click', function(e) {
           if (!searchInput.contains(e.target) && !dropdown.contains(e.target)) {
               dropdown.style.display = 'none';
           }
       });
   </script>"#
}
