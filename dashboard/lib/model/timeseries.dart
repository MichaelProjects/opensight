import 'package:fl_chart/fl_chart.dart';
import 'package:intl/intl.dart';

class Timeseries {
  List<FlSpot> value;
  List<String> date;

  Timeseries(this.value, this.date);

  factory Timeseries.newObject() {
    return Timeseries([], []);
  }
  factory Timeseries.fromJson(Map data) {
    List<String> dates = [];
    List<FlSpot> values = [];
    double counter = 0;
    int before = 0;
    for (Map day in data["data"]["data"]) {
      DateFormat format = DateFormat("yyyy-MM-dd");
      DateTime dayDate = format.parse(day["day"]);
      if (before > dayDate.day) {
        dates.add("${dayDate.day}.${matchMonth(dayDate.month)}");
      } else {
        dates.add(dayDate.day.toString());
      }
      values.add(FlSpot(counter, day["counter"]));
      counter += 1;
      before = dayDate.day;
    }
    return Timeseries(values, dates);
  }
}

String matchMonth(int month) {
  switch (month) {
    case 0:
      return "Jan";
    case 1:
      return "Feb";
    case 2:
      return "Mar";
    case 3:
      return "Apr";
    case 4:
      return "May";
    case 5:
      return "Jun";
    case 6:
      return "Jul";
    case 7:
      return "Aug";
    case 8:
      return "Sep";
    case 9:
      return "Oct";
    case 10:
      return "Nov";
    case 11:
      return "Dec";
    default:
      return "";
  }
}
