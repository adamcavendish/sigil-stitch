import com.example.auth.Authenticatable;
import com.example.base.BaseService;
import com.example.serial.Serializable;

public class AdminService extends BaseService implements Authenticatable, Serializable {
    public boolean isAdmin() {
        return true;
    }
}
