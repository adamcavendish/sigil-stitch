package repo

// Repository defines data access methods.
type Repository interface {
	FindByID(id string) (Entity, error)

	Save(entity Entity) error
}
