import { Authenticatable } from './auth';
import { BaseService } from './base';
import type { Serializable } from './serial';

export class AdminService extends BaseService implements Authenticatable, Serializable {
  isAdmin(): boolean {
    return true
  }
}
