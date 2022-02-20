import 'package:fl_chart/fl_chart.dart';
import 'package:flutter/material.dart';

class LineChartCard extends StatefulWidget {
  List<FlSpot> data;
  LineChartCard(this.data, {Key? key}) : super(key: key);
  @override
  _LineChartCardState createState() => _LineChartCardState();
}

class _LineChartCardState extends State<LineChartCard> {
  double biggest = 0;
  double smallest = 0;
  @override
  Widget build(BuildContext context) {
    for (var x in widget.data) {
      if (x.y > biggest) {
        biggest = x.y;
      }
      if (x.x < smallest) {
        smallest = x.x;
      }
    }
    print(smallest);

    LineChartBarData _lineBarData(List<FlSpot> chartData) {
      return LineChartBarData(
        colors: const [Color(0xFFFF26B5), Color(0xFFFF5B5B)],
        dotData: FlDotData(show: false),
        belowBarData: BarAreaData(
          show: true,
          colors: const [Color(0x10FF26B5), Color(0x00FF26B5)],
          gradientFrom: const Offset(0.5, 0),
          gradientTo: const Offset(0.5, 1),
        ),
        spots: chartData,
      );
    }

    LineChartData mainData(List<FlSpot> lalh) {
      return LineChartData(
          lineBarsData: [_lineBarData(lalh)],
          maxY: biggest,
          minX: 19,
          gridData: FlGridData(show: false),
          borderData: FlBorderData(show: false));
    }

    return SizedBox(
        child: Padding(
            padding: const EdgeInsets.only(right: 18, top: 10, bottom: 18),
            child: widget.data.isEmpty == true
                ? CircularProgressIndicator()
                : LineChart(mainData(widget.data))));
  }
}
