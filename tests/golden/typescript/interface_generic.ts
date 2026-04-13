export interface Repository<T> {
  findById(id: string): Promise<T>;

  save(entity: T): Promise<void>;
}
