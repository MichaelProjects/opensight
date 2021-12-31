class Urls {
  static const String host = 'https://app-dev.fynancial.de';

  // V1 Core Endpoints
  static Uri getAllApplications = Uri.parse('$host/core/v1/application');
  static Uri getApplication = Uri.parse('$host/core/v1/application');
  static Uri login = Uri.parse('$host/core/v1/login');

  // V1 Analytics Endpoints
}
