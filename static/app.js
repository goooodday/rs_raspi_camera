class CameraStreamClient {
    constructor() {
        this.websocket = null;
        this.isConnected = false;
        this.connectionStartTime = null;
        
        // Statistics
        this.stats = {
            framesReceived: 0,
            totalDataReceived: 0,
            lastFrameTime: 0,
            frameTimestamps: [],
            connectionTime: 0
        };
        
        // DOM elements
        this.elements = {
            statusDot: document.getElementById('statusDot'),
            statusText: document.getElementById('statusText'),
            connectionTime: document.getElementById('connectionTime'),
            videoStream: document.getElementById('videoStream'),
            loadingText: document.getElementById('loadingText'),
            connectBtn: document.getElementById('connectBtn'),
            disconnectBtn: document.getElementById('disconnectBtn'),
            fullscreenBtn: document.getElementById('fullscreenBtn'),
            fpsValue: document.getElementById('fpsValue'),
            framesReceived: document.getElementById('framesReceived'),
            latency: document.getElementById('latency'),
            dataReceived: document.getElementById('dataReceived'),
            errorMessage: document.getElementById('errorMessage')
        };
        
        this.setupEventListeners();
        this.updateConnectionTime();
        
        // Auto-connect on page load
        this.connect();
    }
    
    setupEventListeners() {
        this.elements.connectBtn.addEventListener('click', () => this.connect());
        this.elements.disconnectBtn.addEventListener('click', () => this.disconnect());
        this.elements.fullscreenBtn.addEventListener('click', () => this.toggleFullscreen());
        
        // Handle visibility change to pause/resume connection
        document.addEventListener('visibilitychange', () => {
            if (document.hidden && this.isConnected) {
                console.log('Page hidden, maintaining connection but reducing activity');
            } else if (!document.hidden && this.isConnected) {
                console.log('Page visible, resuming full activity');
            }
        });
    }
    
    connect() {
        if (this.isConnected) {
            console.log('Already connected');
            return;
        }
        
        this.hideError();
        this.updateStatus('연결 중...', false);
        this.elements.connectBtn.disabled = true;
        
        // Determine WebSocket URL
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const wsUrl = `${protocol}//${window.location.host}/ws`;
        
        console.log(`Connecting to WebSocket: ${wsUrl}`);
        
        try {
            this.websocket = new WebSocket(wsUrl);
            this.setupWebSocketHandlers();
        } catch (error) {
            console.error('Failed to create WebSocket connection:', error);
            this.showError('연결 생성 실패: ' + error.message);
            this.updateStatus('연결 실패', false);
            this.elements.connectBtn.disabled = false;
        }
    }
    
    setupWebSocketHandlers() {
        this.websocket.onopen = () => {
            console.log('WebSocket connected');
            this.isConnected = true;
            this.connectionStartTime = Date.now();
            this.updateStatus('연결됨', true);
            this.elements.connectBtn.disabled = true;
            this.elements.disconnectBtn.disabled = false;
            this.hideError();
        };
        
        this.websocket.onmessage = (event) => {
            try {
                const message = JSON.parse(event.data);
                this.handleMessage(message);
            } catch (error) {
                console.error('Failed to parse message:', error);
                this.showError('메시지 파싱 오류: ' + error.message);
            }
        };
        
        this.websocket.onerror = (error) => {
            console.error('WebSocket error:', error);
            this.showError('WebSocket 오류가 발생했습니다');
        };
        
        this.websocket.onclose = (event) => {
            console.log('WebSocket closed:', event.code, event.reason);
            this.isConnected = false;
            this.connectionStartTime = null;
            this.updateStatus('연결 끊어짐', false);
            this.elements.connectBtn.disabled = false;
            this.elements.disconnectBtn.disabled = true;
            this.elements.videoStream.style.display = 'none';
            this.elements.loadingText.style.display = 'block';
            this.elements.loadingText.textContent = '연결이 끊어졌습니다';
            
            if (event.code !== 1000 && event.code !== 1001) {
                this.showError(`연결이 끊어졌습니다 (코드: ${event.code})`);
                // Auto-reconnect after 3 seconds for unexpected disconnections
                setTimeout(() => {
                    if (!this.isConnected) {
                        console.log('Attempting auto-reconnect...');
                        this.connect();
                    }
                }, 3000);
            }
        };
    }
    
    handleMessage(message) {
        if (message.message_type === 'frame' && message.data) {
            this.handleFrame(message.data, message.timestamp);
        } else {
            console.log('Received message:', message);
        }
    }
    
    handleFrame(base64Data, timestamp) {
        // Update statistics
        this.stats.framesReceived++;
        this.stats.totalDataReceived += base64Data.length;
        
        const currentTime = Date.now();
        const latency = currentTime - timestamp;
        
        // Update frame timestamps for FPS calculation
        this.stats.frameTimestamps.push(currentTime);
        // Keep only last 30 frame timestamps
        if (this.stats.frameTimestamps.length > 30) {
            this.stats.frameTimestamps.shift();
        }
        
        // Display the frame
        const imageData = `data:image/jpeg;base64,${base64Data}`;
        this.elements.videoStream.src = imageData;
        this.elements.videoStream.style.display = 'block';
        this.elements.loadingText.style.display = 'none';
        
        // Update UI statistics
        this.updateStatistics(latency);
    }
    
    updateStatistics(latency) {
        // Calculate FPS
        const now = Date.now();
        const timestamps = this.stats.frameTimestamps.filter(t => now - t < 1000);
        const fps = timestamps.length;
        
        // Update DOM
        this.elements.fpsValue.textContent = fps;
        this.elements.framesReceived.textContent = this.stats.framesReceived.toLocaleString();
        this.elements.latency.textContent = `${latency}ms`;
        
        // Convert bytes to MB
        const dataMB = (this.stats.totalDataReceived * 0.75 / 1024 / 1024).toFixed(2); // 0.75 for base64 overhead
        this.elements.dataReceived.textContent = `${dataMB} MB`;
    }
    
    disconnect() {
        if (this.websocket && this.isConnected) {
            console.log('Disconnecting WebSocket');
            this.websocket.close(1000, 'User requested disconnect');
        }
    }
    
    updateStatus(text, connected) {
        this.elements.statusText.textContent = text;
        if (connected) {
            this.elements.statusDot.classList.add('connected');
        } else {
            this.elements.statusDot.classList.remove('connected');
        }
    }
    
    updateConnectionTime() {
        if (this.connectionStartTime) {
            const elapsed = Date.now() - this.connectionStartTime;
            const seconds = Math.floor(elapsed / 1000);
            const minutes = Math.floor(seconds / 60);
            const hours = Math.floor(minutes / 60);
            
            const timeString = `${hours.toString().padStart(2, '0')}:${(minutes % 60).toString().padStart(2, '0')}:${(seconds % 60).toString().padStart(2, '0')}`;
            this.elements.connectionTime.textContent = timeString;
        } else {
            this.elements.connectionTime.textContent = '--:--:--';
        }
        
        // Update every second
        setTimeout(() => this.updateConnectionTime(), 1000);
    }
    
    toggleFullscreen() {
        const videoContainer = document.querySelector('.video-container');
        
        if (document.fullscreenElement) {
            document.exitFullscreen().catch(err => {
                console.error('Error exiting fullscreen:', err);
            });
        } else {
            videoContainer.requestFullscreen().catch(err => {
                console.error('Error entering fullscreen:', err);
                this.showError('전체화면 모드를 사용할 수 없습니다');
            });
        }
    }
    
    showError(message) {
        this.elements.errorMessage.textContent = message;
        this.elements.errorMessage.style.display = 'block';
        
        // Auto-hide error after 5 seconds
        setTimeout(() => this.hideError(), 5000);
    }
    
    hideError() {
        this.elements.errorMessage.style.display = 'none';
    }
}

// Initialize the camera stream client when the page loads
document.addEventListener('DOMContentLoaded', () => {
    console.log('Initializing Camera Stream Client...');
    window.cameraClient = new CameraStreamClient();
});

// Handle page unload
window.addEventListener('beforeunload', () => {
    if (window.cameraClient && window.cameraClient.isConnected) {
        window.cameraClient.disconnect();
    }
});
