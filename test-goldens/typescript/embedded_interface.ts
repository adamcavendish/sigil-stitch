import type { BaseUser } from './base';
import type { AdminRole } from './roles';

export interface AdminUser {
  BaseUser;
  AdminRole;
  permissions: string[];
}
