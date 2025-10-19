/* GCodeKit2 Web Pendant - Real-time Client JavaScript */

class PendantClient {
    constructor(config) {
        this.config = config;
        this.ws = null;
        this.connected = false;
        this.messageCount = 0;
        this.init();
    }

    init() {
        this.connectWebSocket();
        this.setupEventListeners();
        this.startStatusPolling();
    }

    connectWebSocket() {
        try {
            this.ws = new WebSocket(this.config.wsUrl);
            
            this.ws.onopen = () => {
                this.connected = true;
                this.updateConnectionStatus(true);
                console.log('WebSocket connected');
            };

            this.ws.onmessage = (event) => {
                this.handleMessage(JSON.parse(event.data));
            };

            this.ws.onerror = (error) => {
                console.error('WebSocket error:', error);
            };

            this.ws.onclose = () => {
                this.connected = false;
                this.updateConnectionStatus(false);
                console.log('WebSocket disconnected');
                setTimeout(() => this.connectWebSocket(), 3000);
            };
        } catch (error) {
            console.error('Failed to connect WebSocket:', error);
        }
    }

    setupEventListeners() {
        // Jog buttons
        document.querySelectorAll('.jog-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                const axis = e.target.dataset.axis;
                const distance = parseFloat(e.target.dataset.distance);
                this.sendJog(axis, distance);
            });
        });

        // Emergency stop
        const emergencyBtn = document.getElementById('emergencyStop');
        if (emergencyBtn) {
            emergencyBtn.addEventListener('click', () => this.emergencyStop());
        }
    }

    startStatusPolling() {
        setInterval(() => this.pollStatus(), 1000);
    }

    pollStatus() {
        fetch(`${this.config.apiUrl}/status`)
            .then(response => response.json())
            .then(data => this.updateStatus(data))
            .catch(error => console.error('Status poll error:', error));
    }

    updateStatus(status) {
        // Update position display
        const posX = document.getElementById('posX');
        const posY = document.getElementById('posY');
        const posZ = document.getElementById('posZ');
        const state = document.getElementById('state');

        if (posX) posX.textContent = status.pos_x.toFixed(2);
        if (posY) posY.textContent = status.pos_y.toFixed(2);
        if (posZ) posZ.textContent = status.pos_z.toFixed(2);
        
        if (state) {
            state.textContent = status.state.charAt(0).toUpperCase() + status.state.slice(1);
            state.className = `state ${status.state.toLowerCase()}`;
        }
    }

    sendJog(axis, distance) {
        const payload = {
            axis: axis,
            distance: distance,
            feed_rate: 100
        };

        fetch(`${this.config.apiUrl}/jog`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(payload)
        })
        .catch(error => console.error('Jog command error:', error));
    }

    emergencyStop() {
        if (confirm('Confirm Emergency Stop?')) {
            fetch(`${this.config.apiUrl}/emergency-stop`, {
                method: 'POST'
            })
            .catch(error => console.error('Emergency stop error:', error));
        }
    }

    handleMessage(message) {
        this.messageCount++;
        this.updateMessageCount();

        switch (message.type) {
            case 'status':
                this.updateStatus(message.data);
                break;
            case 'error':
                console.error('Server error:', message.message);
                break;
        }
    }

    updateConnectionStatus(connected) {
        const statusElement = document.getElementById('connection-status');
        if (statusElement) {
            if (connected) {
                statusElement.textContent = 'Connected';
                statusElement.classList.remove('disconnected');
                statusElement.classList.add('connected');
            } else {
                statusElement.textContent = 'Disconnected';
                statusElement.classList.remove('connected');
                statusElement.classList.add('disconnected');
            }
        }
    }

    updateMessageCount() {
        const countElement = document.getElementById('message-count');
        if (countElement) {
            countElement.textContent = `${this.messageCount} messages`;
        }
    }
}

// Initialize pendant client when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    window.pendantClient = new PendantClient(config);
});
