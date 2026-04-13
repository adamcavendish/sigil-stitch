import type { AdminAccount, GuestAccount, ServiceAccount, UserAccount } from './models';

type Account = UserAccount
| AdminAccount
| ServiceAccount
| GuestAccount;
