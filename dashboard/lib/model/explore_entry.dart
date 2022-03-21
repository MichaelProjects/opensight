import 'package:dashboard/model/analytic.dart';

import 'session_entry.dart';

class ExploreEntry {
  List<SessionEntry> session;
  List<AnalyticEntry> analytic;

  ExploreEntry(this.session, this.analytic);

  factory ExploreEntry.newObject() {
    return ExploreEntry([], []);
  }
  factory ExploreEntry.fromJson(Map response) {
    List<SessionEntry> session = [];
    List<AnalyticEntry> analytic = [];

    for (Map item in response["data"]) {
      AnalyticEntry data = AnalyticEntry.fromJson(item);
      analytic.add(data);
    }
    return ExploreEntry(session, analytic);
  }
}
