import type { User } from './models';

/**
 * Service for managing users.
 */
export class UserService {
  private userRepo: UserRepository;
  private readonly logger: Logger;

  async getUser(id: string): Promise<User> {
    return this.userRepo.findById(id)
  }
}
