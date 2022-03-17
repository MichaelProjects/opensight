import 'package:dashboard/model/timeseries.dart';
import 'package:fl_chart/fl_chart.dart';
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';

class LineChartCard extends StatefulWidget {
  Timeseries seriesData;
  LineChartCard(this.seriesData, {Key? key}) : super(key: key);
  @override
  _LineChartCardState createState() => _LineChartCardState();
}

class _LineChartCardState extends State<LineChartCard> {
  double biggest = 0;
  double smallest = 0;
  @override
  Widget build(BuildContext context) {
    for (var x in widget.seriesData.value) {
      if (x.y > biggest) {
        biggest = x.y;
      }
      if (x.x < smallest || smallest == 0) {
        smallest = x.x;
      }
    }

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
          titlesData: FlTitlesData(
            rightTitles: SideTitles(showTitles: false),
            leftTitles: SideTitles(
              showTitles: true,
              getTextStyles: (context, value) => const TextStyle(fontSize: 10),
            ),
            bottomTitles: SideTitles(
              interval: (widget.seriesData.value.length / 14),
              getTextStyles: (context, value) => const TextStyle(fontSize: 10),
              getTitles: (values) {
                return widget.seriesData.date[values.toInt()];
              },
              showTitles: true,
            ),
            topTitles: SideTitles(showTitles: false),
          ),
          lineBarsData: [_lineBarData(lalh)],
          maxY: biggest,
          minX: 0,
          gridData: FlGridData(show: false),
          borderData: FlBorderData(show: false));
    }

    return SizedBox(
        child: Padding(
            padding: const EdgeInsets.only(right: 18, top: 10, bottom: 18),
            child: widget.seriesData.value.isEmpty == true
                ? CircularProgressIndicator()
                : LineChart(mainData(widget.seriesData.value))));
  }
}
