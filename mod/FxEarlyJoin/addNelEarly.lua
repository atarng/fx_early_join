Include("Common")

function addNelEarly()
	if TryVariable("G_所持_NelJoinsEarly_c") == false then
		Log("addNelEarly] Nel Joined!")
		VariableEntry("G_所持_NelJoinsEarly_c", 1)
		UnitJoinSilent("PID_エル")
		UnitJoinSilent("PID_ラファール")
		UnitJoinSilent("PID_セレスティア")
		UnitJoinSilent("PID_グレゴリー")
		UnitJoinSilent("PID_マデリーン")
	end
end
