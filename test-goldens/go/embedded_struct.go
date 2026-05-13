package models

// UserAdmin composes User and Admin.
type UserAdmin struct {
	User
	Admin
	Role string `json:"role"`
}
