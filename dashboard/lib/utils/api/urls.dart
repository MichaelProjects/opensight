class Urls {
  static const String host = 'https://app-dev.fynancial.de';

  // V1 Core Endpoints
  static Uri getAllApplications = Uri.parse('$host/core/v1/application');
  static Uri getApplication = Uri.parse('$host/core/v1/application');
  static Uri login = Uri.parse('$host/core/v1/login');
  static Uri createApplication = Uri.parse('$host/core/v1/application');

  // V1 Analytics Endpoints
  static Uri getAllAnalytics(String appId) {
    return Uri.parse('$host/analytic/v1/$appId/session');
  }
}
