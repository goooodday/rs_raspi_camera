#!/bin/bash

# Deployment script for rs_raspi_camera
# Usage: ./deploy_to_pi.sh [username] [pi_ip]
# Example: ./deploy_to_pi.sh pi 192.168.1.100

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
USERNAME="${1:-pi}"
PI_IP="${2}"
BINARY_PATH="target/aarch64-unknown-linux-gnu/release/rs_raspi_camera"

echo -e "${BLUE}🚀 rs_raspi_camera Deployment Script${NC}"
echo -e "${BLUE}=================================${NC}"

# Show usage if help is requested or no IP provided
if [ "$1" = "-h" ] || [ "$1" = "--help" ] || [ -z "$PI_IP" ]; then
    echo "Usage: $0 [username] <pi_ip>"
    echo "Deploy rs_raspi_camera to Raspberry Pi"
    echo ""
    echo "Arguments:"
    echo "  username    SSH username for the Pi (default: pi)"
    echo "  pi_ip       IP address of your Raspberry Pi (required)"
    echo ""
    echo "Examples:"
    echo "  $0 192.168.1.100          # Use default username 'pi'"
    echo "  $0 pi 192.168.1.100       # Specify username 'pi'"
    echo "  $0 ubuntu 192.168.1.100   # Use username 'ubuntu'"
    exit 0
fi

echo -e "${BLUE}📍 Target: ${YELLOW}$USERNAME@$PI_IP${NC}"

# Check if binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${RED}❌ Binary not found at $BINARY_PATH${NC}"
    echo -e "${YELLOW}💡 Please build first with: ./scripts/build_for_pi.sh${NC}"
    exit 1
fi

# Deploy the binary
echo -e "${BLUE}📦 Deploying rs_raspi_camera binary...${NC}"
scp $BINARY_PATH $USERNAME@$PI_IP:~/rs_raspi_camera

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Binary deployed successfully${NC}"
else
    echo -e "${RED}❌ Failed to deploy binary${NC}"
    exit 1
fi

# Set permissions and provide instructions
echo -e "${BLUE}🔧 Setting up on Raspberry Pi...${NC}"
ssh $USERNAME@$PI_IP << EOF
    chmod +x rs_raspi_camera
    echo "✅ rs_raspi_camera is ready to run!"
EOF

echo ""
echo -e "${GREEN}🎉 Deployment completed successfully!${NC}"
echo ""
echo -e "${BLUE}📋 Next steps on your Raspberry Pi:${NC}"
echo -e "1. SSH to your Pi:"
echo -e "   ${YELLOW}ssh $USERNAME@$PI_IP${NC}"
echo ""
echo -e "2. Run the web application:"
echo -e "   ${YELLOW}./rs_raspi_camera${NC}"
echo ""
echo -e "3. Access the web application:"
echo -e "   ${GREEN}http://$PI_IP:3000${NC}          (Main page)"
echo -e "   ${GREEN}http://$PI_IP:3000/api/hello${NC} (API test)"
echo -e "   ${GREEN}http://$PI_IP:3000/status${NC}    (System status)"
echo ""
echo -e "${BLUE}🔧 To run as a service (optional):${NC}"
echo -e "   See README.md for systemd service setup instructions"