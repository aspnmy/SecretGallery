# 使用多阶段构建来创建最小的Next.js镜像

# 阶段1：构建Next.js应用
FROM node:18-alpine AS builder

# 设置工作目录
WORKDIR /app

# 复制package.json和package-lock.json
COPY package*.json ./

# 安装依赖
RUN npm ci

# 复制源代码
COPY . .

# 构建应用
RUN npm run build

# 阶段2：创建生产镜像
FROM caddy:2-alpine AS production

# 复制构建产物到Caddy的静态目录
COPY --from=builder /app/.next/static /usr/share/caddy/_next/static
COPY --from=builder /app/out /usr/share/caddy

# 复制Caddy配置文件
COPY Caddyfile /etc/caddy/Caddyfile

# 暴露端口
EXPOSE 80
EXPOSE 443

# 启动Caddy
CMD ["caddy", "run", "--config", "/etc/caddy/Caddyfile", "--adapter", "caddyfile"]
