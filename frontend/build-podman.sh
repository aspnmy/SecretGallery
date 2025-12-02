#!/bin/bash

# 前端Podman构建脚本
# 使用多阶段构建创建最小的Next.js + Caddy镜像

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 脚本名称
SCRIPT_NAME="$(basename "$0")"

# 显示帮助信息
show_help() {
    echo -e "${GREEN}前端Podman构建脚本${NC}"
    echo -e "使用多阶段构建创建最小的Next.js + Caddy镜像，自动配置HTTPS"
    echo -e "\n用法: ${YELLOW}$SCRIPT_NAME${NC} [选项]"
    echo -e "\n选项:"
    echo -e "  ${YELLOW}build${NC}    构建Docker镜像"
    echo -e "  ${YELLOW}run${NC}      运行Docker容器"
    echo -e "  ${YELLOW}clean${NC}    清理构建产物和容器"
    echo -e "  ${YELLOW}help${NC}     显示帮助信息"
    echo -e "\n示例:"
    echo -e "  ${YELLOW}$SCRIPT_NAME${NC} build        # 构建镜像"
    echo -e "  ${YELLOW}$SCRIPT_NAME${NC} run          # 运行容器"
    echo -e "  ${YELLOW}$SCRIPT_NAME${NC} clean        # 清理资源"
}

# 构建镜像
build_image() {
    echo -e "${GREEN}=== 开始构建前端镜像 ===${NC}"
    
    # 检查Podman是否安装
    if ! command -v podman &> /dev/null; then
        echo -e "${RED}错误: Podman 未安装${NC}"
        echo -e "请先安装 Podman: https://podman.io/getting-started/installation"
        exit 1
    fi
    
    # 构建镜像
    echo -e "${YELLOW}正在构建镜像...${NC}"
    podman build -t gcm-frontend:latest .
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}镜像构建成功: gcm-frontend:latest${NC}"
    else
        echo -e "${RED}镜像构建失败${NC}"
        exit 1
    fi
}

# 运行容器
run_container() {
    echo -e "${GREEN}=== 开始运行前端容器 ===${NC}"
    
    # 检查Podman是否安装
    if ! command -v podman &> /dev/null; then
        echo -e "${RED}错误: Podman 未安装${NC}"
        echo -e "请先安装 Podman: https://podman.io/getting-started/installation"
        exit 1
    fi
    
    # 检查镜像是否存在
    if ! podman image exists gcm-frontend:latest; then
        echo -e "${YELLOW}镜像不存在，正在构建...${NC}"
        build_image
    fi
    
    # 停止并删除现有容器
    if podman container exists gcm-frontend; then
        echo -e "${YELLOW}停止并删除现有容器...${NC}"
        podman stop gcm-frontend &> /dev/null
        podman rm gcm-frontend &> /dev/null
    fi
    
    # 运行容器
    echo -e "${YELLOW}正在运行容器...${NC}"
    podman run -d \
        --name gcm-frontend \
        -p 80:80 \
        -p 443:443 \
        --restart unless-stopped \
        gcm-frontend:latest
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}容器运行成功: gcm-frontend${NC}"
        echo -e "访问地址: https://localhost${NC}"
        echo -e "HTTP会自动重定向到HTTPS${NC}"
    else
        echo -e "${RED}容器运行失败${NC}"
        exit 1
    fi
}

# 清理资源
clean_resources() {
    echo -e "${GREEN}=== 开始清理资源 ===${NC}"
    
    # 检查Podman是否安装
    if ! command -v podman &> /dev/null; then
        echo -e "${RED}错误: Podman 未安装${NC}"
        echo -e "请先安装 Podman: https://podman.io/getting-started/installation"
        exit 1
    fi
    
    # 停止并删除容器
    if podman container exists gcm-frontend; then
        echo -e "${YELLOW}停止并删除容器...${NC}"
        podman stop gcm-frontend &> /dev/null
        podman rm gcm-frontend &> /dev/null
    fi
    
    # 删除镜像
    if podman image exists gcm-frontend:latest; then
        echo -e "${YELLOW}删除镜像...${NC}"
        podman rmi gcm-frontend:latest &> /dev/null
    fi
    
    echo -e "${GREEN}资源清理完成${NC}"
}

# 主函数
main() {
    # 检查参数
    if [ $# -eq 0 ]; then
        show_help
        exit 1
    fi
    
    case "$1" in
        build)
            build_image
            ;;
        run)
            run_container
            ;;
        clean)
            clean_resources
            ;;
        help)
            show_help
            ;;
        *)
            echo -e "${RED}错误: 未知选项 '$1'${NC}"
            show_help
            exit 1
            ;;
    esac
}

# 执行主函数
main "$@"
