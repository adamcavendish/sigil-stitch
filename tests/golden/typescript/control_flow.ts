import type { NotFoundError } from './errors';

export function validate(input: string): boolean {
  if (input.length === 0) {
    throw new NotFoundError('empty input');
  } else if (input.length > 100) {
    return false;
  } else {
    return true;
  }
}
