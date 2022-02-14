import 'package:dashboard/controllers/dashboard/analytics_model.dart';
import 'package:fl_chart/fl_chart.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class LineChartCard extends StatefulWidget {
  List<FlSpot> data;
  LineChartCard(this.data);

  @override
  State<LineChartCard> createState() => _LineChartCardState();
}

class _LineChartCardState extends State<LineChartCard> {
  @override
  Widget build(BuildContext context) {
    AnalyticModel analyticsController = Provider.of<AnalyticModel>(context);
    return SizedBox(
        child: Padding(
            padding: const EdgeInsets.only(right: 18, top: 10, bottom: 18),
            child: LineChart(
              LineChartData(
                lineBarsData: [
                  LineChartBarData(
                    colors: const [Color(0xFFFF26B5), Color(0xFFFF5B5B)],
                    dotData: FlDotData(show: false),
                    belowBarData: BarAreaData(
                      show: true,
                      colors: const [Color(0x10FF26B5), Color(0x00FF26B5)],
                      gradientFrom: const Offset(0.5, 0),
                      gradientTo: const Offset(0.5, 1),
                    ),
                    spots: widget.data,
                  )
                ],
                maxY: 200,
                gridData: FlGridData(show: false),
                borderData: FlBorderData(show: false),
              ),
            )));
  }
}

LineChartData _mainData() {
  return LineChartData(
    maxY: 200,
    gridData: FlGridData(show: false),
    borderData: FlBorderData(show: false),
    lineBarsData: [_lineBarData()],
  );
}

LineChartBarData _lineBarData() {
  return LineChartBarData(
    spots: _values,
    colors: const [Color(0xFFFF26B5), Color(0xFFFF5B5B)],
    colorStops: const [0.25, 0.5, 0.75],
    gradientFrom: const Offset(0.5, 0),
    gradientTo: const Offset(0.5, 1),
    barWidth: 2,
    isStrokeCapRound: true,
    dotData: FlDotData(show: false),
    belowBarData: BarAreaData(
      show: true,
      colors: const [Color(0x10FF26B5), Color(0x00FF26B5)],
      gradientFrom: const Offset(0.5, 0),
      gradientTo: const Offset(0.5, 1),
    ),
  );
}
