import com.example.base.BaseService
import com.example.serial.Serializable

class AdminService extends BaseService with Serializable {
  def isAdmin(): Boolean = {
    true
  }
}
