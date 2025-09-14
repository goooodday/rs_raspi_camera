use crate::system_info::SystemInfo;
use axum::response::Html;

/// Hello World HTML page with Korean support
pub async fn hello_world() -> Html<String> {
    let system_info = SystemInfo::detect();

    let html_content = format!(
        r#"
    <!DOCTYPE html>
    <html lang="ko">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Hello World - {}</title>
        <style>
            body {{
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                margin: 0;
                padding: 0;
                height: 100vh;
                display: flex;
                justify-content: center;
                align-items: center;
                color: white;
            }}
            .container {{
                text-align: center;
                background: rgba(255, 255, 255, 0.1);
                backdrop-filter: blur(10px);
                border-radius: 20px;
                padding: 40px;
                box-shadow: 0 8px 32px 0 rgba(31, 38, 135, 0.37);
                border: 1px solid rgba(255, 255, 255, 0.18);
                max-width: 700px;
                margin: 20px;
            }}
            h1 {{
                font-size: 3.5em;
                margin-bottom: 20px;
                text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
                animation: fadeInUp 1s ease-out;
            }}
            .subtitle {{
                font-size: 1.3em;
                margin-bottom: 30px;
                opacity: 0.9;
                animation: fadeInUp 1s ease-out 0.3s both;
            }}
            .info-card {{
                background: rgba(255, 255, 255, 0.1);
                border-radius: 15px;
                padding: 25px;
                margin: 20px 0;
                animation: fadeInUp 1s ease-out 0.6s both;
            }}
            .api-button {{
                display: inline-block;
                background: rgba(255, 255, 255, 0.2);
                color: white;
                text-decoration: none;
                padding: 15px 30px;
                border-radius: 25px;
                border: 2px solid rgba(255, 255, 255, 0.3);
                transition: all 0.3s ease;
                margin: 10px;
                font-weight: bold;
                animation: fadeInUp 1s ease-out 0.9s both;
            }}
            .api-button:hover {{
                background: rgba(255, 255, 255, 0.3);
                transform: translateY(-2px);
                box-shadow: 0 4px 12px rgba(0,0,0,0.2);
            }}
            @keyframes fadeInUp {{
                from {{
                    opacity: 0;
                    transform: translateY(30px);
                }}
                to {{
                    opacity: 1;
                    transform: translateY(0);
                }}
            }}
            .platform-icon {{
                font-size: 5em;
                margin-bottom: 20px;
                animation: bounce 2s infinite;
            }}
            @keyframes bounce {{
                0%, 20%, 50%, 80%, 100% {{
                    transform: translateY(0);
                }}
                40% {{
                    transform: translateY(-10px);
                }}
                60% {{
                    transform: translateY(-5px);
                }}
            }}
            .system-info {{
                display: grid;
                grid-template-columns: 1fr 1fr;
                gap: 20px;
                margin-top: 20px;
            }}
            .info-item {{
                background: rgba(255, 255, 255, 0.05);
                padding: 15px;
                border-radius: 10px;
                border: 1px solid rgba(255, 255, 255, 0.1);
            }}
            .info-label {{
                font-size: 0.9em;
                opacity: 0.8;
                margin-bottom: 5px;
            }}
            .info-value {{
                font-size: 1.1em;
                font-weight: bold;
            }}
            @media (max-width: 600px) {{
                .system-info {{
                    grid-template-columns: 1fr;
                }}
                h1 {{
                    font-size: 2.5em;
                }}
                .container {{
                    padding: 20px;
                }}
            }}
        </style>
    </head>
    <body>
        <div class="container">
            <div class="platform-icon">{}</div>
            <h1>Hello World!</h1>
            <div class="subtitle">
                안녕하세요! {}에서 실행되는 Rust 웹 서버입니다.
            </div>
            
            <div class="info-card">
                <h3>📹 Rust Camera Server</h3>
                <p>이 웹페이지는 Rust와 Axum 프레임워크로 만들어졌습니다.</p>
                <p>{} 카메라를 사용한 실시간 스트리밍을 지원합니다.</p>
            </div>
            
            <div class="system-info">
                <div class="info-item">
                    <div class="info-label">현재 시간</div>
                    <div class="info-value" id="time"></div>
                </div>
                <div class="info-item">
                    <div class="info-label">서버 상태</div>
                    <div class="info-value">🟢 정상 가동</div>
                </div>
                <div class="info-item">
                    <div class="info-label">포트</div>
                    <div class="info-value">3000</div>
                </div>
                <div class="info-item">
                    <div class="info-label">프레임워크</div>
                    <div class="info-value">Axum + Tokio</div>
                </div>
            </div>
            
            <div style="margin-top: 30px;">
                <a href="/camera" class="api-button">📹 실시간 카메라</a>
                <a href="/api/hello" class="api-button">API 테스트</a>
                <a href="/status" class="api-button">시스템 상태</a>
            </div>
        </div>
        
        <script>
            function updateTime() {{
                const now = new Date();
                document.getElementById('time').textContent = now.toLocaleString('ko-KR');
            }}
            updateTime();
            setInterval(updateTime, 1000);
        </script>
    </body>
    </html>
    "#,
        system_info.platform_name, // title
        system_info.platform_icon, // platform icon
        system_info.platform_name, // subtitle
        system_info.platform_name  // info card
    );

    Html(html_content)
}

/// System status endpoint with dynamic system detection
pub async fn system_status() -> Html<String> {
    let system_info = SystemInfo::detect();

    let html_content = format!(
        r#"
    <!DOCTYPE html>
    <html lang="ko">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>System Status - {}</title>
        <style>
            body {{
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                background: linear-gradient(135deg, #2c3e50 0%, #3498db 100%);
                margin: 0;
                padding: 20px;
                color: white;
                min-height: 100vh;
            }}
            .container {{
                max-width: 1000px;
                margin: 0 auto;
                background: rgba(255, 255, 255, 0.1);
                backdrop-filter: blur(10px);
                border-radius: 20px;
                padding: 30px;
                box-shadow: 0 8px 32px 0 rgba(31, 38, 135, 0.37);
                border: 1px solid rgba(255, 255, 255, 0.18);
            }}
            h1 {{
                text-align: center;
                margin-bottom: 40px;
                font-size: 2.5em;
                text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
            }}
            .status-grid {{
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
                gap: 20px;
                margin-bottom: 30px;
            }}
            .status-card {{
                background: rgba(255, 255, 255, 0.1);
                border-radius: 15px;
                padding: 20px;
                border: 1px solid rgba(255, 255, 255, 0.2);
            }}
            .status-card h3 {{
                margin-top: 0;
                color: #ecf0f1;
                border-bottom: 2px solid rgba(255, 255, 255, 0.2);
                padding-bottom: 10px;
            }}
            .status-item {{
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin: 15px 0;
                padding: 10px;
                background: rgba(255, 255, 255, 0.05);
                border-radius: 8px;
            }}
            .status-value {{
                font-weight: bold;
                color: #2ecc71;
            }}
            .compilation-badge {{
                background: {};
                color: white;
                padding: 4px 8px;
                border-radius: 12px;
                font-size: 0.8em;
                margin-left: 8px;
            }}
            .home-button {{
                display: inline-block;
                background: rgba(255, 255, 255, 0.2);
                color: white;
                text-decoration: none;
                padding: 12px 24px;
                border-radius: 25px;
                border: 2px solid rgba(255, 255, 255, 0.3);
                transition: all 0.3s ease;
                margin: 10px;
                text-align: center;
            }}
            .home-button:hover {{
                background: rgba(255, 255, 255, 0.3);
                transform: translateY(-2px);
            }}
        </style>
    </head>
    <body>
        <div class="container">
            <h1>📊 시스템 상태 - rs_raspi_camera</h1>
            
            <div class="status-grid">
                <div class="status-card">
                    <h3>💻 서버 정보</h3>
                    <div class="status-item">
                        <span>프로젝트:</span>
                        <span class="status-value">rs_raspi_camera</span>
                    </div>
                    <div class="status-item">
                        <span>서버 상태:</span>
                        <span class="status-value">🟢 정상 가동</span>
                    </div>
                    <div class="status-item">
                        <span>포트:</span>
                        <span class="status-value">3000</span>
                    </div>
                    <div class="status-item">
                        <span>업타임:</span>
                        <span class="status-value" id="uptime">계산 중...</span>
                    </div>
                </div>
                
                <div class="status-card">
                    <h3>🦀 Rust 정보</h3>
                    <div class="status-item">
                        <span>런타임:</span>
                        <span class="status-value">Tokio</span>
                    </div>
                    <div class="status-item">
                        <span>웹 프레임워크:</span>
                        <span class="status-value">Axum 0.8.4</span>
                    </div>
                    <div class="status-item">
                        <span>비동기 모델:</span>
                        <span class="status-value">활성화</span>
                    </div>
                    <div class="status-item">
                        <span>컴파일 모드:</span>
                        <span class="status-value">{}<span class="compilation-badge">{}</span></span>
                    </div>
                </div>
                
                <div class="status-card">
                    <h3>{} {}</h3>
                    <div class="status-item">
                        <span>플랫폼:</span>
                        <span class="status-value">{}</span>
                    </div>
                    <div class="status-item">
                        <span>아키텍처:</span>
                        <span class="status-value">{}</span>
                    </div>
                    <div class="status-item">
                        <span>운영체제:</span>
                        <span class="status-value">{}</span>
                    </div>
                    <div class="status-item">
                        <span>타겟:</span>
                        <span class="status-value">{}</span>
                    </div>
                    <div class="status-item">
                        <span>네트워크:</span>
                        <span class="status-value">{}</span>
                    </div>
                </div>
                
                <div class="status-card">
                    <h3>🔗 API 엔드포인트</h3>
                    <div class="status-item">
                        <span>/ (Home):</span>
                        <span class="status-value">활성</span>
                    </div>
                    <div class="status-item">
                        <span>/api/hello:</span>
                        <span class="status-value">활성</span>
                    </div>
                    <div class="status-item">
                        <span>/status:</span>
                        <span class="status-value">활성</span>
                    </div>
                    <div class="status-item">
                        <span>/static/*:</span>
                        <span class="status-value">활성</span>
                    </div>
                </div>
            </div>
            
            <div style="text-align: center;">
                <a href="/" class="home-button">🏠 홈으로 돌아가기</a>
                <a href="/api/hello" class="home-button">🔧 API 테스트</a>
            </div>
        </div>
        
        <script>
            let startTime = Date.now();
            function updateUptime() {{
                const now = Date.now();
                const uptime = Math.floor((now - startTime) / 1000);
                const hours = Math.floor(uptime / 3600);
                const minutes = Math.floor((uptime % 3600) / 60);
                const seconds = uptime % 60;
                
                document.getElementById('uptime').textContent = 
                    `${{hours}}시간 ${{minutes}}분 ${{seconds}}초`;
            }}
            
            updateUptime();
            setInterval(updateUptime, 1000);
        </script>
    </body>
    </html>
    "#,
        system_info.platform_name, // title
        if system_info.is_cross_compiled {
            "#e67e22"
        } else {
            "#27ae60"
        }, // badge color
        system_info.get_compilation_info(),
        if system_info.is_cross_compiled {
            "CROSS"
        } else {
            "NATIVE"
        },
        system_info.platform_icon,
        system_info.platform_name,
        system_info.platform_name,
        system_info.arch,
        system_info.os.to_uppercase(),
        system_info.target,
        system_info.get_network_info()
    );

    Html(html_content)
}

/// Camera page HTML
pub async fn camera_page() -> Html<String> {
    let system_info = SystemInfo::detect();

    let html_content = format!(
        r#"
    <!DOCTYPE html>
    <html lang="ko">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>실시간 카메라 - {}</title>
        <style>
            body {{
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                background: linear-gradient(135deg, #2c3e50 0%, #3498db 100%);
                margin: 0;
                padding: 20px;
                color: white;
                min-height: 100vh;
            }}
            .container {{
                max-width: 1200px;
                margin: 0 auto;
                background: rgba(255, 255, 255, 0.1);
                backdrop-filter: blur(10px);
                border-radius: 20px;
                padding: 30px;
                box-shadow: 0 8px 32px 0 rgba(31, 38, 135, 0.37);
                border: 1px solid rgba(255, 255, 255, 0.18);
            }}
            h1 {{
                text-align: center;
                margin-bottom: 30px;
                font-size: 2.5em;
                text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
            }}
            .camera-container {{
                text-align: center;
                margin-bottom: 30px;
            }}
            .camera-feed {{
                max-width: 100%;
                height: auto;
                border-radius: 15px;
                box-shadow: 0 4px 20px rgba(0,0,0,0.3);
                background: #000;
                min-height: 300px;
                display: block;
                margin: 0 auto;
            }}
            .status-info {{
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
                gap: 20px;
                margin-top: 30px;
            }}
            .status-card {{
                background: rgba(255, 255, 255, 0.1);
                border-radius: 15px;
                padding: 20px;
                border: 1px solid rgba(255, 255, 255, 0.2);
            }}
            .status-card h3 {{
                margin-top: 0;
                color: #ecf0f1;
                border-bottom: 2px solid rgba(255, 255, 255, 0.2);
                padding-bottom: 10px;
            }}
            .status-item {{
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin: 10px 0;
                padding: 8px;
                background: rgba(255, 255, 255, 0.05);
                border-radius: 8px;
            }}
            .status-value {{
                font-weight: bold;
                color: #2ecc71;
            }}
            .nav-buttons {{
                text-align: center;
                margin-top: 30px;
            }}
            .nav-button {{
                display: inline-block;
                background: rgba(255, 255, 255, 0.2);
                color: white;
                text-decoration: none;
                padding: 12px 24px;
                border-radius: 25px;
                border: 2px solid rgba(255, 255, 255, 0.3);
                transition: all 0.3s ease;
                margin: 10px;
            }}
            .nav-button:hover {{
                background: rgba(255, 255, 255, 0.3);
                transform: translateY(-2px);
            }}
        </style>
    </head>
    <body>
        <div class="container">
            <h1>📹 실시간 카메라 스트리밍</h1>
            
            <div class="camera-container">
                <img id="camera-feed" class="camera-feed" 
                     src="/api/camera/stream" 
                     alt="카메라 피드 로딩 중..."
                     onload="updateStatus('connected')"
                     onerror="updateStatus('error')">
            </div>
            
            <div class="status-info">
                <div class="status-card">
                    <h3>📹 카메라 상태</h3>
                    <div class="status-item">
                        <span>연결 상태:</span>
                        <span class="status-value" id="connection-status">연결 중...</span>
                    </div>
                    <div class="status-item">
                        <span>해상도:</span>
                        <span class="status-value">640x480</span>
                    </div>
                    <div class="status-item">
                        <span>프레임레이트:</span>
                        <span class="status-value">~30 FPS</span>
                    </div>
                    <div class="status-item">
                        <span>포맷:</span>
                        <span class="status-value">MJPEG</span>
                    </div>
                </div>
                
                <div class="status-card">
                    <h3>🛠️ 시스템 정보</h3>
                    <div class="status-item">
                        <span>서버:</span>
                        <span class="status-value">Rust Axum</span>
                    </div>
                    <div class="status-item">
                        <span>카메라 백엔드:</span>
                        <span class="status-value" id="camera-backend">로딩 중...</span>
                    </div>
                    <div class="status-item">
                        <span>스트리밍:</span>
                        <span class="status-value">실시간</span>
                    </div>
                    <div class="status-item">
                        <span>지연시간:</span>
                        <span class="status-value">< 100ms</span>
                    </div>
                    <div class="status-item">
                        <span>플랫폼:</span>
                        <span class="status-value" id="platform-info">{} {}</span>
                    </div>
                </div>
            </div>
            
            <div class="nav-buttons">
                <a href="/" class="nav-button">🏠 홈으로</a>
                <a href="/status" class="nav-button">📊 시스템 상태</a>
                <a href="/api/camera/status" class="nav-button">🔧 카메라 API</a>
            </div>
        </div>
        
        <script>
            function updateStatus(status) {{
                const statusElement = document.getElementById('connection-status');
                if (status === 'connected') {{
                    statusElement.textContent = '🟢 연결됨';
                    statusElement.style.color = '#2ecc71';
                    // Fetch camera backend info
                    fetchCameraInfo();
                }} else if (status === 'error') {{
                    statusElement.textContent = '🔴 연결 실패';
                    statusElement.style.color = '#e74c3c';
                }}
            }}
            
            // Fetch camera backend information
            async function fetchCameraInfo() {{
                try {{
                    const response = await fetch('/api/camera/status');
                    const data = await response.json();
                    const backendElement = document.getElementById('camera-backend');
                    
                    if (data.camera && data.camera.backend) {{
                        const mode = data.camera.camera_mode || 'Unknown';
                        let displayText = '';
                        let color = '#2ecc71';
                        
                        switch(mode) {{
                            case 'OpenCV':
                                displayText = '📹 실제 카메라';
                                color = '#2ecc71';
                                break;
                            case 'Simulation':
                                displayText = '🎭 시뮬레이션';
                                color = '#f39c12';
                                break;
                            case 'Error':
                                displayText = '❌ 오류';
                                color = '#e74c3c';
                                break;
                            default:
                                displayText = '🔄 초기화 중';
                                color = '#3498db';
                        }}
                        
                        backendElement.textContent = displayText;
                        backendElement.style.color = color;
                    }}
                }} catch (error) {{
                    console.error('Failed to fetch camera info:', error);
                    const backendElement = document.getElementById('camera-backend');
                    backendElement.textContent = '⚠️ 상태 불명';
                    backendElement.style.color = '#e74c3c';
                }}
            }}
            
            // Update camera info periodically
            setInterval(fetchCameraInfo, 5000);
            
            // Refresh camera feed periodically if it fails
            setInterval(() => {{
                const img = document.getElementById('camera-feed');
                if (img.complete && img.naturalHeight === 0) {{
                    img.src = '/api/camera/stream?' + new Date().getTime();
                }}
            }}, 5000);
        </script>
    </body>
    </html>
    "#,
        system_info.platform_name, // title
        system_info.platform_icon, // platform icon in system info
        system_info.platform_name  // platform name in system info
    );

    Html(html_content)
}
