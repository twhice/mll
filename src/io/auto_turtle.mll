repeat {
	set a1=sorter1.config
}until a1 != null
repeat {
	set a2=sorter2.config
}until a2!=null
repeat {
	set a3=sorter3.config
}until a3!=null
def ieic(i){
	repeat {ubind(@poly)}
	until @unit != null
	ulocate(core,0,cx,cy,_,core)
	return core.a3 > 200
}
set b = get_link(id)
set ac = b.ammoCapacity
if ac != null{
	set i = null
	
	if ieic(a1) != 0{
		set i = a1
	}elif ieic(a2)==1{
		set i = a2
	}elif ieic(a3) == 1{
		set i = a3
	}
	if i!=null{
		set x = b.x
		set y = b.y
		while b.dead == 0 && a.ammo < ac-5{
			while @unit==null || @unit.dead == 1 || @unit.flag!=0{
				ubind(@poly)
			}
			set ui = u.firstItem
			if ui!=i{
				umove(cx,cy)
				if ui!=null{
					uidrop(core,999)
				}
				uitake(core,i,999)
			}else{
				umove(x,y)
				uidrop(b,999)
			}
		}
	}
}
set id=(id+1)%@links
