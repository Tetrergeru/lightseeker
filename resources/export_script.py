import bpy
from mathutils import Matrix

ob = list(filter(lambda it: it.name == 'Cone', bpy.context.scene.objects))[0]
arm = ob.find_armature()
mesh = ob.to_mesh()
anim = arm.animation_data

def export_skl(arm, fname):
    with open(fname, "w") as file:
        for bone in arm.pose.bones:
            file.write("b %s\n" % bone.name)
            if bone.parent:
                file.write("bp %s\n" % bone.parent.name)
            file.write("bb ")
            for a in matrix_decompose(bone.bone.matrix_local):
                file.write("%f " % a)
            file.write("\n\n")

def export_skin(mesh, arm, fname):
    i = 1
    bones = {}
    for bone in arm.pose.bones:
        bones[bone.name] = i
        i += 1
    with open(fname, "w") as file:
        for v in mesh.vertices:
            file.write("vl %d " % (v.index + 1))
            for g in v.groups:
                if not ob.vertex_groups[g.group].name in bones:
                    continue
                file.write("%s %f " % (bones[ob.vertex_groups[g.group].name], g.weight))
            file.write("\n")

def export_anim(arm, fname):
    with open(fname, "w") as file:
        for i in range(0, 30):
            j = i * 2
            bpy.context.scene.frame_set(j)
            file.write("fr %d\n" % (j + 1))
            for bone in arm.pose.bones:
                file.write("af ")
                m = bone.matrix_basis
                print(bone.head)
                m = Matrix.Translation(bone.head) @ m @ Matrix.Translation(-bone.head)
#                m = arm.convert_space(pose_bone=bone, 
#                    matrix=bone.matrix_basis,
#                    from_space='LOCAL',
#                    to_space='POSE',
#                )
#                print(m)
                for a in matrix_decompose(m):
                    file.write("%f " % a)
                file.write("\n")
                print()
            file.write("\n")

EPS = 0.0001

def matrix_decompose(mat):
    tr, rot, sc = mat.decompose()
    rot = rot.to_euler("XYZ")
    result = [rot[0], rot[2], rot[1]]
    if abs(tr[0]) > EPS and abs(tr[1]) > EPS and abs(tr[2]) > EPS:
        result.extend([tr[0], tr[2], tr[1]])
    if abs(sc[0]) > 1 + EPS and abs(sc[1]) > 1 + EPS and abs(sc[2]) > 1 + EPS:
        result.extend([tr[0], tr[2], tr[1]])
        result.extend([sc[0], sc[2], sc[1]])
    return result

#export_skl(arm, "bell.skl")
#export_skin(mesh, arm, "bell.skin")
export_anim(arm, "bell.anim")