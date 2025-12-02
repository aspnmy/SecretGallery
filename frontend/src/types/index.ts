// 资源类型
export type ResourceType = {
  id: number;
  title: string;
  title_en: string;
  description: string;
  resource_type: string;
  poster_image: string;
  images: string[];
  links: {
    magnet: string[];
    ed2k: string[];
    uc: string[];
    mobile: string[];
    tianyi: string[];
    quark: string[];
    '115': string[];
    aliyun: string[];
    pikpak: string[];
    baidu: string[];
    '123': string[];
    xunlei: string[];
    online: string[];
    others: string[];
  };
  tmdb_id: number | null;
  stickers: string[];
  media_type: 'video' | 'image';
  created_at: string;
  updated_at: string;
  liked_by: number[];
  is_approved: boolean;
};

// 用户类型
export type User = {
  id: number;
  username: string;
  email: string;
  is_admin: boolean;
  created_at: string;
  updated_at: string;
};

// 登录请求
export type LoginRequest = {
  username: string;
  password: string;
};

// 注册请求
export type RegisterRequest = {
  username: string;
  email: string;
  password: string;
};

// 登录响应
export type LoginResponse = {
  access_token: string;
  user: User;
};

// 资源查询参数
export type ResourceQuery = {
  type?: 'video' | 'image';
  page?: number;
  limit?: number;
  search?: string;
  sort?: 'created_at' | 'updated_at' | 'title';
  order?: 'asc' | 'desc';
};

// 资源提交请求
export type ResourceSubmitRequest = {
  title: string;
  title_en: string;
  description: string;
  resource_type: string;
  poster_image: string;
  images: string[];
  links: {
    magnet: string[];
    ed2k: string[];
    uc: string[];
    mobile: string[];
    tianyi: string[];
    quark: string[];
    '115': string[];
    aliyun: string[];
    pikpak: string[];
    baidu: string[];
    '123': string[];
    xunlei: string[];
    online: string[];
    others: string[];
  };
  tmdb_id: number | null;
  media_type: 'video' | 'image';
};

// 分页响应
export type PaginatedResponse<T> = {
  data: T[];
  total: number;
  page: number;
  limit: number;
  pages: number;
};

// 响应通用类型
export type ApiResponse<T> = {
  success: boolean;
  data: T;
  message?: string;
  error?: string;
};

// 图片类型
export type Image = {
  id: string;
  src: string;
  width: number;
  height: number;
  alt: string;
  href?: string;
};

// 相册类型
export type AlbumType = 'video' | 'image';

// 设置类型
export type Setting = {
  key: string;
  value: string;
  description: string;
  type: string;
};

// TMDB 配置类型
export type TmdbConfig = {
  enabled: boolean;
  api_key: string;
  api_url: string;
};

// 搜索结果类型
export type SearchResult = {
  id: number;
  title: string;
  title_en: string;
  poster_path: string;
  release_date: string;
  media_type: 'movie' | 'tv';
};

// 视频播放器配置类型
export type VideoPlayerConfig = {
  src: string;
  poster?: string;
  autoPlay?: boolean;
  muted?: boolean;
  controls?: boolean;
  preload?: 'auto' | 'metadata' | 'none';
  crossOrigin?: 'anonymous' | 'use-credentials';
  playsInline?: boolean;
  fullscreen?: boolean;
  pip?: boolean;
  playbackRate?: number;
};

// 图片压缩配置类型
export type ImageCompressorConfig = {
  quality?: number;
  maxWidth?: number;
  maxHeight?: number;
  minWidth?: number;
  minHeight?: number;
  convertSize?: number;
  convertType?: string;
  strict?: boolean;
  checkOrientation?: boolean;
  success?: (result: File) => void;
  error?: (err: Error) => void;
};