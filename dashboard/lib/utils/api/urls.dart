class Urls {
  static const String host = 'https://app-dev.fynancial.de';

  // V1 Core Endpoints
  static Uri getAllApplications = Uri.parse('$host/core/v1/application');

  static Uri getApplication(String appId) {
    return Uri.parse('$host/core/v1/application/$appId');
  }

  static Uri login = Uri.parse('$host/core/v1/login');
  static Uri createApplication = Uri.parse('$host/core/v1/application');

  // V1 Analytics Endpoints
  static Uri getAllAnalytics(String appId) {
    return Uri.parse('$host/analytic/v1/$appId/session');
  }

  static Uri getAnalyseNewUser(String appid, int startFrame, int endFrame) {
    return Uri.parse(
        '$host/analytic/v1/$appid/analyse/user/new?start=$startFrame&end=$endFrame');
  }

  static Uri getAnalyseUser(String appid, int startFrame, int endFrame) {
    return Uri.parse(
        '$host/analytic/v1/$appid/analyse/user?start=$startFrame&end=$endFrame');
  }

  static Uri getAnalyseDisplaySize(String appid, int startFrame, int endFrame) {
    return Uri.parse(
        '$host/analytic/v1/$appid/analyse/display/size?start=$startFrame&end=$endFrame');
  }

  static Uri getSessionLengthHistory(
      String appid, int startFrame, int endFrame) {
    return Uri.parse(
        '$host/analytic/v1/$appid/analyse/user/session?start=$startFrame&end=$endFrame');
  }
}
