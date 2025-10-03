# 🎥 라즈베리 파이 카메라 스트림

라즈베리 파이에서 실시간 카메라 영상을 웹브라우저로 스트리밍하는 Rust 애플리케이션입니다.

## ✨ 기능

- **실시간 비디오 스트리밍**: 라즈베리 파이 카메라의 영상을 실시간으로 웹브라우저에 전송
- **WebSocket 통신**: 낮은 지연시간으로 고품질 스트리밍
- **반응형 웹 인터페이스**: 모바일과 데스크톱 모두 지원
- **성능 모니터링**: FPS, 지연시간, 데이터 사용량 등 실시간 통계
- **자동 재연결**: 연결이 끊어져도 자동으로 재연결 시도
- **전체화면 지원**: 풀스크린 모드로 영상 시청 가능

## 🚀 시작하기

두 가지 방법으로 애플리케이션을 빌드하고 실행할 수 있습니다.

1.  **라즈베리 파이에서 직접 빌드 (권장)**: 가장 간단하고 확실한 방법입니다.
2.  **Docker를 이용한 크로스 컴파일**: 개발 PC에서 라즈베리 파이용 실행 파일을 빌드합니다.

---

### 방법 1: 라즈베리 파이에서 직접 빌드 (권장)

이 방법은 라즈베리 파이에서 직접 소스 코드를 컴파일합니다.

#### 1. 라즈베리 파이 설정

```bash
# 시스템 업데이트
sudo apt update && sudo apt upgrade -y

# 카메라 모듈 활성화 (raspi-config 사용)
sudo raspi-config
# -> 3 Interface Options -> I1 Camera -> Yes 선택 후 재부팅

# 빌드에 필요한 패키지 설치
sudo apt install -y build-essential pkg-config libopencv-dev libclang-dev clang v4l-utils
```

#### 2. Rust 설치

```bash
# 라즈베리 파이 터미널에서 실행
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

#### 3. 프로젝트 빌드 및 실행

```bash
# GitHub에서 소스 코드 클론
git clone https://github.com/<your-github-username>/raspberry_pi_camera_stream.git
cd raspberry_pi_camera_stream

# 프로젝트 빌드 (최적화된 릴리스 버전)
cargo build --release

# 실행
./target/release/raspberry_pi_camera_stream
```

#### 4. 웹브라우저에서 접속

다른 컴퓨터나 스마트폰의 웹 브라우저에서 `http://<라즈베리파이_IP>:3030` 주소로 접속하세요.

---

### 방법 2: Docker를 이용한 크로스 컴파일 (고급)

이 방법은 개발 PC(macOS, Linux, Windows)에서 Docker를 사용하여 라즈베리 파이(ARM64)용 실행 파일을 빌드합니다. 개발 PC에 Rust나 C++ 툴체인, OpenCV 라이브러리를 직접 설치할 필요가 없어 개발 환경을 깨끗하게 유지할 수 있습니다.

#### 1. 사전 준비: Docker 환경 구성

- **Docker 설치**: 먼저 PC에 Docker Desktop을 설치해야 합니다. [공식 홈페이지](https://www.docker.com/products/docker-desktop/)에서 자신의 운영체제에 맞는 버전을 다운로드하여 설치하세요.

- **Docker 데몬 실행**: Docker 관련 명령어를 사용하려면 Docker Desktop 애플리케이션이 실행 중이어야 합니다. Docker가 실행되고 있는지 확인하세요.

- **(선택) Colima 사용자**: macOS에서 Colima를 사용하는 경우, `colima start` 명령어로 Docker 데몬을 시작해야 할 수 있습니다.

#### 2. 최초 1회: Docker 빌드 이미지 생성

프로젝트 루트 디렉토리에서 아래 명령어를 실행하여 `aarch64` 빌드 환경을 포함한 Docker 이미지를 생성합니다. **이 과정은 최초 한 번만 실행하면 되며,** 향후 빌드부터는 재사용됩니다.

```bash
# aarch64 아키텍처용 빌드 환경을 포함한 Docker 이미지를 생성합니다.
docker build --platform linux/arm64 -t raspberry-pi-camera-stream .
```

#### 3. 개발 워크플로우: 프로젝트 컴파일

로컬 PC에서 소스 코드를 수정한 후, 아래 명령어만 실행하면 Docker 컨테이너 내부에서 프로젝트가 컴파일됩니다. `-v "$(pwd):/app"` 옵션은 현재 프로젝트 디렉토리를 컨테이너의 `/app` 디렉토리와 실시간으로 동기화합니다.

```bash
# Docker 컨테이너를 실행하여 프로젝트를 빌드합니다.
docker run --rm -v "$(pwd):/app" -w /app --platform linux/arm64 raspberry-pi-camera-stream
```
> **참고**: 최초 컴파일은 모든 의존성을 다운로드하므로 시간이 걸리지만, 이후부터는 변경된 코드만 컴파일하므로 훨씬 빠르게 완료됩니다.

#### 4. 실행 파일 배포

컴파일이 성공적으로 완료되면, target/release/ `raspberry_pi_camera_stream` 실행 파일을 찾을 수 있습니다.

```bash
# scp를 이용해 컴파일된 바이너리를 라즈베리 파이로 전송
scp target/release/raspberry_pi_camera_stream pi@<라즈베리파이_IP>:~

# 라즈베리 파이에 SSH로 접속하여 실행 권한을 부여하고 실행
ssh pi@<라즈베리파이_IP>
chmod +x ./raspberry_pi_camera_stream
./raspberry_pi_camera_stream
```

---

## ⚙️ 구성 옵션

환경 변수를 통해 애플리케이션을 구성할 수 있습니다.

- `RUST_LOG`: 로그 레벨 설정 ( `debug`, `info`, `warn`, `error` )
- 서버 포트 및 카메라 해상도 등은 `src/main.rs` 파일에서 직접 수정할 수 있습니다.

## 🛠️ 문제 해결

### 카메라가 인식되지 않는 경우
- `sudo raspi-config`를 통해 카메라가 활성화되었는지 다시 확인하고 재부팅하세요.
- `sudo usermod -a -G video $USER` 명령어로 현재 사용자를 `video` 그룹에 추가한 후 재로그인하세요.

### 네트워크 접속 문제
- `hostname -I` 명령어로 라즈베리 파이의 IP 주소를 확인하세요.
- `sudo ufw status`로 방화벽 설정을 확인하고, `sudo ufw allow 3030`으로 포트를 허용해주세요.

## 🙏 감사의 말

- [OpenCV](https://opencv.org/)
- [Tokio](https://tokio.rs/)
- [Warp](https://github.com/seanmonstar/warp)
