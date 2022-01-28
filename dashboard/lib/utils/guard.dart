class AuthGuard {
  static bool canUserAccessPage() {
    return true;
  }

  bool canUserAccessDashboard() {
    return false;
  }
}
