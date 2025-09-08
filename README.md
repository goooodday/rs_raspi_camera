# 🎥 라즈베리 파이 카메라 스트림

라즈베리 파이에서 실시간 카메라 영상을 웹브라우저로 스트리밍하는 Rust 애플리케이션입니다.

## ✨ 기능

- **실시간 비디오 스트리밍**: 라즈베리 파이 카메라의 영상을 실시간으로 웹브라우저에 전송
- **WebSocket 통신**: 낮은 지연시간으로 고품질 스트리밍
- **반응형 웹 인터페이스**: 모바일과 데스크톱 모두 지원
- **성능 모니터링**: FPS, 지연시간, 데이터 사용량 등 실시간 통계
- **자동 재연결**: 연결이 끊어져도 자동으로 재연결 시도
- **전체화면 지원**: 풀스크린 모드로 영상 시청 가능

## 🚀 빠른 시작

### 1. 라즈베리 파이 설정

```bash
# 시스템 업데이트
sudo apt update && sudo apt upgrade -y

# 카메라 모듈 활성화
sudo raspi-config
# -> Interface Options -> Camera -> Enable

# 필요한 패키지 설치
sudo apt install -y \
    build-essential \
    pkg-config \
    libopencv-dev \
    libclang-dev \
    v4l-utils
```

### 2. Rust 설치 (라즈베리 파이에서)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 3. 프로젝트 클론 및 빌드

```bash
git clone <your-repository-url>
cd raspberry_pi_camera_stream

# 빌드
cargo build --release
```

### 4. 실행

```bash
# 환경 변수 설정 (옵션)
export RUST_LOG=info

# 실행
./target/release/raspberry_pi_camera_stream
```

### 5. 웹브라우저에서 접속

브라우저에서 `http://<라즈베리파이IP>:3030`으로 접속

## 🔧 크로스 컴파일 (권장)

개발 컴퓨터에서 라즈베리 파이용 바이너리를 컴파일할 수 있습니다.

### macOS/Linux에서 크로스 컴파일

```bash
# ARM 타겟 추가
rustup target add armv7-unknown-linux-gnueabihf  # Raspberry Pi 3/4
# 또는
rustup target add aarch64-unknown-linux-gnu      # Raspberry Pi 4 (64-bit)

# 크로스 컴파일 도구 설치 (macOS)
brew install arm-linux-gnueabihf-gcc

# 크로스 컴파일 도구 설치 (Ubuntu/Debian)
sudo apt install gcc-arm-linux-gnueabihf

# .cargo/config.toml 파일 생성
mkdir -p .cargo
cat > .cargo/config.toml << EOF
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"

[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
EOF

# 빌드
cargo build --release --target armv7-unknown-linux-gnueabihf
```

### 바이너리를 라즈베리 파이로 전송

```bash
# SCP를 이용한 파일 전송
scp target/armv7-unknown-linux-gnueabihf/release/raspberry_pi_camera_stream pi@<라즈베리파이IP>:~/

# SSH로 접속하여 실행
ssh pi@<라즈베리파이IP>
chmod +x raspberry_pi_camera_stream
./raspberry_pi_camera_stream
```

## ⚙️ 구성 옵션

환경 변수를 통해 애플리케이션을 구성할 수 있습니다:

```bash
# 로그 레벨 설정
export RUST_LOG=debug  # debug, info, warn, error

# 서버 포트 (기본값: 3030)
# main.rs에서 수정 필요

# 카메라 설정 (기본값: 640x480@30fps)
# main.rs에서 수정 필요
```

## 🛠️ 개발 및 디버깅

### 로컬 개발 (웹캠 사용)

```bash
# 웹캠이 연결된 개발 컴퓨터에서 테스트
cargo run
```

### 카메라 테스트

```bash
# 카메라 장치 확인
v4l2-ctl --list-devices

# 카메라 테스트 촬영
raspistill -o test.jpg

# 비디오 테스트
raspivid -o test.h264 -t 5000
```

### 포트 확인

```bash
# 포트가 사용 중인지 확인
sudo netstat -tlnp | grep 3030

# 방화벽 설정 (필요시)
sudo ufw allow 3030
```

## 📁 프로젝트 구조

```
raspberry_pi_camera_stream/
├── src/
│   ├── main.rs          # 메인 애플리케이션
│   ├── camera.rs        # 카메라 캡처 모듈
│   └── server.rs        # 웹서버 및 WebSocket 서버
├── static/
│   ├── index.html       # 웹 클라이언트 UI
│   └── app.js          # WebSocket 클라이언트 로직
├── Cargo.toml          # Rust 의존성 설정
└── README.md           # 이 파일
```

## 🔍 트러블슈팅

### 카메라 관련 문제

1. **카메라가 인식되지 않는 경우**
   ```bash
   # 카메라 모듈 활성화 확인
   sudo raspi-config
   
   # 리부팅
   sudo reboot
   ```

2. **권한 문제**
   ```bash
   # 사용자를 video 그룹에 추가
   sudo usermod -a -G video $USER
   # 재로그인 필요
   ```

### 네트워크 관련 문제

1. **다른 기기에서 접속이 안 되는 경우**
   ```bash
   # 라즈베리 파이 IP 확인
   hostname -I
   
   # 방화벽 확인
   sudo ufw status
   ```

2. **포트 충돌**
   ```bash
   # 다른 포트 사용 (main.rs 수정)
   let server_port = 8080;
   ```

### 성능 최적화

1. **GPU 메모리 할당 늘리기**
   ```bash
   sudo raspi-config
   # -> Advanced Options -> Memory Split -> 128
   ```

2. **CPU 오버클럭** (주의: 쿨링 필요)
   ```bash
   # /boot/config.txt 편집
   sudo nano /boot/config.txt
   # 다음 줄 추가:
   # arm_freq=1750
   # over_voltage=6
   ```

## 🤝 기여하기

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 라이선스

이 프로젝트는 MIT 라이선스 하에 배포됩니다.

## 🙏 감사의 말

- [OpenCV](https://opencv.org/) - 컴퓨터 비전 라이브러리
- [Tokio](https://tokio.rs/) - 비동기 런타임
- [Warp](https://github.com/seanmonstar/warp) - 웹 프레임워크
- [Raspberry Pi Foundation](https://www.raspberrypi.org/) - 훌륭한 하드웨어

## 📞 지원

문제가 발생하면 GitHub Issues를 통해 문의해 주세요.
