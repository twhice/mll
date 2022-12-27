set_enabled(switch1,0)
# 检测是否开机
# 新方法
repeat {
    set start = switch1.enabled
} until start == 1 
# 检测是否已经选择弹药 并且读取配置
repeat {
    set ammo  = sorter1.config
} until ammo != null
# 从id==2开始遍历建筑
set i = 2
while i<@links {
    # 链接建筑
    set b = get_link(i)
    # 获取建筑坐标
    set x =  b.x
    set y =  b.y
    # 当填充需求存在时
    while b.ammoCapacity - 5 > b.ammo{
        # 绑定单位
        ubind(@flare)
        # 如果单位拿的物品是弹药,就直接装填
        if @unit.firstItem == ammo{
            umove(x,y)
            uidrop(b,999)
        # 否则去核心
        } else {
            # 定位并移动到核心
            ulocate(core,0,cx,cy,_,co)
            umove(cx,cy)
            # 如果单位不是空手,就仍吧东西扔进核心
            if @unit.firstItem != null {
                uidrop(co,999)
            }
            uitake(co,ammo,999)
        }
    }
    set i = i + 1
}
