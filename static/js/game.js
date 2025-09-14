let playerChain = [document.querySelector('.game-container').dataset.player1Id];
const API_PREFIX = window.location.pathname.includes('/cnxns') ? '/cnxns' : '';

if (!document.getElementById('career-modal')) {
    document.body.insertAdjacentHTML('beforeend', `
        <div id="career-modal" class="career-modal">
            <div class="career-content">
                <h3 id="career-player-name">Player Career</h3>
                <div id="career-table"></div>
                <button onclick="closeCareerModal()">Close</button>
            </div>
        </div>
    `);
}

function updateFooterPosition() {
    const gameContainer = document.querySelector('.game-container');
    const instructionsContainer = document.querySelector('.instructions');
    
    if (!gameContainer || !instructionsContainer) {
        return; // Exit if elements don't exist yet
    }
    
    const gameBottom = gameContainer.offsetTop + gameContainer.offsetHeight;
    const instructionsBottom = instructionsContainer.offsetTop + instructionsContainer.offsetHeight;
    const containerBottom = Math.max(gameBottom, instructionsBottom);
    
    const cornerImages = document.querySelector('.corner-images');
    const bottomStripes = document.querySelector('.bottom-stripes');
    
    if (!cornerImages || !bottomStripes) {
        return; // Exit if footer elements don't exist yet
    }
    
    const imagePosition = containerBottom + 30;
    const stripePosition = imagePosition + 120;
    
    cornerImages.style.top = Math.min(imagePosition, window.innerHeight - 180) + 'px';
    bottomStripes.style.top = Math.min(stripePosition, window.innerHeight - 70) + 'px';
}

function attachSearchListeners(input, dropdown) {
    let searchTimeout;
    
    input.addEventListener('input', function() {
        const query = this.value.trim();
        
        if (query.length < 2) {
            dropdown.style.display = 'none';
            return;
        }

        clearTimeout(searchTimeout);
        searchTimeout = setTimeout(() => {
            fetch(`${API_PREFIX}/api/search?q=${encodeURIComponent(query)}`)
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
                            input.value = player.player_name;
                            dropdown.style.display = 'none';
                            checkPlayerConnection(player.player_id, input);
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
}

function checkPlayerConnection(selectedPlayerId, inputElement) {
    console.log('Checking connection for:', selectedPlayerId);
    
    fetch(`${API_PREFIX}/api/check-connection`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ 
            player_ids_chain: playerChain,
            new_player_id: selectedPlayerId 
        })
    })
    .then(response => response.json())
    .then(data => {
        console.log('Connection response:', data);

        if (!data.success) {
            alert('No shared matches!');
        } else {
            console.log(`Connection found: ${data.shared_matches} ${data.team.color_circles}`);
            lockInPlayer(selectedPlayerId, inputElement, data.shared_matches, data.team.color_circles);
            
            if (data.is_complete) {
                const finalConnectionData = data.final_connection ? {
                    shared_matches: data.final_connection[0],
                    team: { color_circles: data.final_connection[1].color_circles }
                } : null;
                
                completeGame(data.chain_length, finalConnectionData);
            }
        }
    })
    .catch(error => console.error('Connection check error:', error));
}

function lockInPlayer(playerId, inputElement, matchCount, colorCircles) {
    console.log('lockInPlayer called with:', { playerId, matchCount, colorCircles });

    playerChain.push(playerId);
    
    if (matchCount && colorCircles) {
        const playerName = inputElement.value;
        inputElement.value = `${playerName} (${matchCount} ${colorCircles})`;
    }

    inputElement.disabled = true;
    inputElement.style.backgroundColor = '#e9e9e9';
    
    const careerBtn = document.createElement('button');
    careerBtn.className = 'career-btn';
    careerBtn.innerHTML = 'ℹ';
    careerBtn.onclick = () => {
        const playerName = inputElement.value.replace(/ \(\d+ .+\)$/, '');
        showCareerModal(playerId, playerName);
    };

    if (playerChain.length > 1) {
        const removeBtn = document.createElement('button');
        removeBtn.className = 'remove-btn';
        removeBtn.innerHTML = '×';
        removeBtn.onclick = removeLastPlayer;
        inputElement.parentElement.appendChild(removeBtn);
        inputElement.parentElement.appendChild(careerBtn);
    } else {
        inputElement.parentElement.appendChild(careerBtn);
    }
    
    document.querySelectorAll('.remove-btn').forEach((btn, index) => {
        if (index < document.querySelectorAll('.remove-btn').length - 1) {
            btn.remove();
        }
    });
    
    const newInputContainer = document.createElement('div');
    newInputContainer.className = 'input-container';
    newInputContainer.innerHTML = `
        <input type="text" class="connection-input" placeholder="Add another connecting player...">
        <div class="autocomplete-dropdown"></div>
    `;
    
    const currentContainer = inputElement.parentElement;
    currentContainer.parentNode.insertBefore(newInputContainer, currentContainer.nextSibling);
    
    const newInput = newInputContainer.querySelector('.connection-input');
    const newDropdown = newInputContainer.querySelector('.autocomplete-dropdown');
    attachSearchListeners(newInput, newDropdown);
    
    updateFooterPosition();
}

function removeLastPlayer() {
    if (playerChain.length <= 1) return;
    
    fetch(`${API_PREFIX}/api/remove-player`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(playerChain)
    })
    .then(response => response.json())
    .then(data => {
        if (data.success) {
            playerChain = data.updated_chain;
            
            const containers = document.querySelectorAll('.input-container');
            const lastContainer = containers[containers.length - 1];
            if (lastContainer) {
                lastContainer.remove();
            }
            
            const inputs = document.querySelectorAll('.connection-input');
            const lastInput = inputs[inputs.length - 1];
            if (lastInput && lastInput.disabled) {
                lastInput.disabled = false;
                lastInput.style.backgroundColor = '';
                const currentValue = lastInput.value;
                lastInput.value = currentValue.replace(/ \(\d+ .+\)$/, '');
            }
            
            document.querySelectorAll('.remove-btn').forEach(btn => btn.remove());
            document.querySelectorAll('.career-btn').forEach(btn => btn.remove());
            
            if (playerChain.length > 1) {
                const lockedInputs = document.querySelectorAll('.connection-input:disabled');
                const lastLockedInput = lockedInputs[lockedInputs.length - 1];
                if (lastLockedInput) {
                    const removeBtn = document.createElement('button');
                    removeBtn.className = 'remove-btn';
                    removeBtn.innerHTML = '×';
                    removeBtn.onclick = removeLastPlayer;
                    lastLockedInput.parentElement.appendChild(removeBtn);
                    
                    const careerBtn = document.createElement('button');
                    careerBtn.className = 'career-btn';
                    careerBtn.innerHTML = 'ℹ';
                    careerBtn.onclick = () => {
                        const playerName = lastLockedInput.value.replace(/ \(\d+ .+\)$/, '');
                        const playerId = playerChain[playerChain.length - 1];
                        showCareerModal(playerId, playerName);
                    };
                    lastLockedInput.parentElement.appendChild(careerBtn);
                }
            }
            
            updateFooterPosition();
        }
    })
    .catch(error => console.error('Remove error:', error));
}

function completeGame(chainLength, finalConnection) {
    const score = chainLength - 1;

    if (finalConnection && finalConnection.shared_matches && finalConnection.team) {
        const playerBoxes = document.querySelectorAll('.player-box');
        const lastPlayerBox = playerBoxes[playerBoxes.length - 1];
        
        if (lastPlayerBox) {
            const currentText = lastPlayerBox.querySelector('h3').textContent;
            lastPlayerBox.querySelector('h3').textContent = `${currentText} (${finalConnection.shared_matches} ${finalConnection.team.color_circles})`;
        }
    }

    const lastInput = document.querySelector('.connection-input:not(:disabled)');
    if (lastInput) {
        lastInput.parentElement.remove();
    }

    document.querySelectorAll('.remove-btn').forEach(btn => btn.remove());
    document.querySelectorAll('.career-btn').forEach(btn => btn.remove());

    const gameContainer = document.querySelector('.game-container');
    const completionDiv = document.createElement('div');
    completionDiv.innerHTML = `
        <div style="background: #4CAF50; color: white; padding: 20px; border-radius: 12px; text-align: center; margin: 20px 0;">
            <h2>🎉 Completed!</h2>
            <p>You connected the players in ${score} steps!</p>
        </div>
    `;

    gameContainer.appendChild(completionDiv);
    setTimeout(() => {
        updateFooterPosition();
    }, 100);
}

function showCareerModal(playerId, playerName) {
    document.getElementById('career-player-name').textContent = `${playerName} Career`;
    document.getElementById('career-table').innerHTML = 'Loading...';
    document.getElementById('career-modal').style.display = 'block';
    
    fetch(`${API_PREFIX}/api/career?player_id=${playerId}`)
        .then(response => response.json())
        .then(career => {
            let html = '<table><tr><th>Team</th><th>Seasons</th><th>Matches</th></tr>';
            career.forEach(([team, seasons, matches]) => {
                html += `<tr><td>${team}</td><td>${seasons}</td><td>${matches}</td></tr>`;
            });
            html += '</table>';
            document.getElementById('career-table').innerHTML = html;
        })
        .catch(error => {
            document.getElementById('career-table').innerHTML = 'Error loading career data';
        });
}

function closeCareerModal() {
    document.getElementById('career-modal').style.display = 'none';
}

function addCareerButtonToBox(playerBox, playerId, playerName) {
    const careerBtn = document.createElement('button');
    careerBtn.className = 'career-btn';
    careerBtn.innerHTML = 'ℹ';
    careerBtn.onclick = () => showCareerModal(playerId, playerName);
    playerBox.appendChild(careerBtn);
}

const initialInput = document.getElementById('player-search');
const initialDropdown = document.getElementById('autocomplete-dropdown');
attachSearchListeners(initialInput, initialDropdown);

document.addEventListener('click', function(e) {
    document.querySelectorAll('.autocomplete-dropdown').forEach(dropdown => {
        const container = dropdown.parentElement;
        const input = container.querySelector('.connection-input');
        if (!input.contains(e.target) && !dropdown.contains(e.target)) {
            dropdown.style.display = 'none';
        }
    });
});

document.addEventListener('DOMContentLoaded', function() {
    updateFooterPosition();
    
    document.body.insertAdjacentHTML('beforeend', `
        <div id="career-modal" class="career-modal">
            <div class="career-content">
                <h3 id="career-player-name">Player Career</h3>
                <div id="career-table"></div>
                <button onclick="closeCareerModal()">Close</button>
            </div>
        </div>
    `);

    const playerBoxes = document.querySelectorAll('.player-box');
    const gameContainer = document.querySelector('.game-container');
    const player1Id = gameContainer.dataset.player1Id;
    const player2Id = gameContainer.dataset.player2Id;
    
    addCareerButtonToBox(playerBoxes[0], player1Id, playerBoxes[0].querySelector('h3').textContent);
    addCareerButtonToBox(playerBoxes[1], player2Id, playerBoxes[1].querySelector('h3').textContent);
});