class AdminService(BaseService, Authenticatable):
    def is_admin(self) -> bool:
        return True
