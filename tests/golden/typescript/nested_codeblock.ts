import type { User } from './models';

export function getUser(): User {
  const user = new User();
  return user;
}
