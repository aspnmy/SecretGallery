// src/services/authService.ts
// 认证服务，处理用户登录、注册、登出等功能

import axios from 'axios';
import { LoginRequest, RegisterRequest, User } from '@/types';

// 创建axios实例
const apiClient = axios.create({
  baseURL: '/api',
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
});

// 请求拦截器，添加token
apiClient.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

/**
 * 用户登录
 * @param credentials - 登录凭证，包含用户名和密码
 * @returns 登录成功后的用户信息
 */
export const loginUser = async (credentials: LoginRequest): Promise<User> => {
  try {
    const response = await apiClient.post<{ data: User; token: string }>('/auth/login', credentials);
    
    // 保存token和用户信息到本地存储
    localStorage.setItem('token', response.data.token);
    localStorage.setItem('user', JSON.stringify(response.data.data));
    
    return response.data.data;
  } catch (error) {
    console.error('用户登录失败:', error);
    throw error;
  }
};

/**
 * 用户注册
 * @param userData - 注册信息，包含用户名、密码、邮箱等
 * @returns 注册成功后的用户信息
 */
export const registerUser = async (userData: RegisterRequest): Promise<User> => {
  try {
    const response = await apiClient.post<{ data: User }>('/auth/register', userData);
    return response.data.data;
  } catch (error) {
    console.error('用户注册失败:', error);
    throw error;
  }
};

/**
 * 用户登出
 */
export const logoutUser = (): void => {
  // 清除本地存储中的token和用户信息
  localStorage.removeItem('token');
  localStorage.removeItem('user');
  
  // 可以添加其他清理逻辑，如清除状态管理中的用户信息
};

/**
 * 获取当前登录用户信息
 * @returns 当前登录用户信息，如果未登录则返回null
 */
export const getCurrentUser = (): User | null => {
  const userStr = localStorage.getItem('user');
  if (userStr) {
    try {
      return JSON.parse(userStr);
    } catch (error) {
      console.error('解析用户信息失败:', error);
      return null;
    }
  }
  return null;
};

/**
 * 检查用户是否已登录
 * @returns 是否已登录
 */
export const isLoggedIn = (): boolean => {
  return !!localStorage.getItem('token');
};

/**
 * 刷新token
 * @returns 新的token
 */
export const refreshToken = async (): Promise<string> => {
  try {
    const response = await apiClient.post<{ token: string }>('/auth/refresh');
    
    // 保存新token到本地存储
    localStorage.setItem('token', response.data.token);
    
    return response.data.token;
  } catch (error) {
    console.error('刷新token失败:', error);
    // 刷新失败，清除本地存储，强制登出
    logoutUser();
    throw error;
  }
};
